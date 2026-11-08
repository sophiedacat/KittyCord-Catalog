use kittycord_extension_sdk::{
    AppInvocation, AppOutput, Element, Output,
};
use serde::{Deserialize, Serialize};

kittycord_extension_sdk::export!(handle);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RelationLogEntry {
    kind: String,
    user_id: String,
    user_name: Option<String>,
    guild_id: Option<String>,
    guild_name: Option<String>,
    timestamp_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct RelationLoggerState {
    entries: Vec<RelationLogEntry>,
}

fn handle(invocation: AppInvocation) -> AppOutput {
    let mut state: RelationLoggerState = match &invocation.invocation.storage {
        Some(storage) => serde_json::from_str(storage).unwrap_or_default(),
        None => RelationLoggerState::default(),
    };

    match invocation.invocation.action.as_str() {
        "relationship-event" => {
            if let Some(snapshot) = &invocation.app {
                if let Some(relationships) = &snapshot.relationships {
                    for item in &relationships.items {
                        let kind_str = match item.kind {
                            kittycord_extension_sdk::RelationshipKind::Friend => "friend",
                            kittycord_extension_sdk::RelationshipKind::IncomingRequest => "incoming_request",
                            kittycord_extension_sdk::RelationshipKind::OutgoingRequest => "outgoing_request",
                            kittycord_extension_sdk::RelationshipKind::Blocked => "blocked",
                            kittycord_extension_sdk::RelationshipKind::Ignored => "ignored",
                        };
                        if matches!(kind_str, "blocked" | "ignored") {
                            state.entries.push(RelationLogEntry {
                                kind: kind_str.into(),
                                user_id: item.user.id.clone(),
                                user_name: Some(item.user.name.clone()),
                                guild_id: None,
                                guild_name: None,
                                timestamp_ms: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u64,
                            });
                        }
                    }
                }
            }
        }
        "show" => {
            let mut elements = vec![Element::Heading {
                text: "Relation Log".into(),
            }];
            for entry in state.entries.iter().rev().take(50) {
                let location = match (&entry.guild_name, &entry.guild_id) {
                    (Some(name), _) => format!(" in {}", name),
                    (None, Some(id)) => format!(" in guild {}", id),
                    _ => "".into(),
                };
                elements.push(Element::Text {
                    text: format!(
                        "[{} {}]{}",
                        entry.kind,
                        entry.user_name.as_deref().unwrap_or(&entry.user_id),
                        location
                    ),
                });
            }
            elements.push(Element::Separator);
            elements.push(Element::Text {
                text: format!("Total events: {}", state.entries.len()),
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
