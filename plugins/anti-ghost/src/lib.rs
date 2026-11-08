use kittycord_extension_sdk::{
    AppInvocation, AppOutput, Element, MessageEventKind, Output,
};
use serde::{Deserialize, Serialize};

kittycord_extension_sdk::export!(handle);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GhostEntry {
    channel_id: String,
    user_id: String,
    user_name: Option<String>,
    last_message_at: u64,
    read_at: Option<u64>,
    replied_at: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct AntiGhostState {
    ghosts: Vec<GhostEntry>,
}

fn handle(invocation: AppInvocation) -> AppOutput {
    let mut state: AntiGhostState = match &invocation.invocation.storage {
        Some(storage) => serde_json::from_str(storage).unwrap_or_default(),
        None => AntiGhostState::default(),
    };

    match invocation.invocation.action.as_str() {
        "message-event" => {
            if let Some(event) = &invocation.message_event {
                if event.kind == MessageEventKind::Create {
                    if let Some(author_id) = &event.author_id {
                        let existing = state.ghosts.iter_mut().find(|g| g.channel_id == event.channel_id && g.user_id == *author_id);
                        if let Some(ghost) = existing {
                            ghost.last_message_at = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis() as u64;
                        } else {
                            state.ghosts.push(GhostEntry {
                                channel_id: event.channel_id.clone(),
                                user_id: author_id.clone(),
                                user_name: None,
                                last_message_at: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u64,
                                read_at: None,
                                replied_at: None,
                            });
                        }
                        if state.ghosts.len() > 500 {
                            state.ghosts.drain(..state.ghosts.len() - 500);
                        }
                    }
                }
            }
        }
        "read" => {
            if let Some(channel_id) = invocation.invocation.values.get("channel_id") {
                if let Some(ghost) = state.ghosts.iter_mut().find(|g| g.channel_id == *channel_id) {
                    ghost.read_at = Some(std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64);
                }
            }
        }
        "replied" => {
            if let Some(channel_id) = invocation.invocation.values.get("channel_id") {
                if let Some(ghost) = state.ghosts.iter_mut().find(|g| g.channel_id == *channel_id) {
                    ghost.replied_at = Some(std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64);
                }
            }
        }
        "show" => {
            let mut elements = vec![Element::Heading {
                text: "Anti-Ghost".into(),
            }];
            elements.push(Element::Text {
                text: "Shows a ghost icon next to users who messaged you, you read it, but haven't replied.".into(),
            });
            elements.push(Element::Separator);
            let active_ghosts: Vec<_> = state.ghosts.iter().filter(|g| {
                g.read_at.is_some() && g.replied_at.is_none()
            }).collect();
            for ghost in active_ghosts.iter().take(50) {
                elements.push(Element::Text {
                    text: format!("👻 {} ({})", ghost.user_name.as_deref().unwrap_or(&ghost.user_id), ghost.channel_id),
                });
            }
            elements.push(Element::Separator);
            elements.push(Element::Text {
                text: format!("Active ghosts: {}", active_ghosts.len()),
            });
            return AppOutput {
                output: Output { panel: elements, ..Default::default() },
                ..Default::default()
            };
        }
        _ => {}
    }

    let storage = serde_json::to_string(&state).ok();
    AppOutput {
        output: Output { storage, ..Default::default() },
        ..Default::default()
    }
}
