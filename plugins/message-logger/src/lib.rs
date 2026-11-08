use kittycord_extension_sdk::{
    AppInvocation, AppOutput, Element, MessageEventKind, Output,
};
use serde::{Deserialize, Serialize};

kittycord_extension_sdk::export!(handle);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LogEntry {
    kind: String,
    channel_id: String,
    message_id: String,
    author_id: Option<String>,
    content: Option<String>,
    timestamp_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct LoggerState {
    entries: Vec<LogEntry>,
}

fn handle(invocation: AppInvocation) -> AppOutput {
    let mut state: LoggerState = match &invocation.invocation.storage {
        Some(storage) => serde_json::from_str(storage).unwrap_or_default(),
        None => LoggerState::default(),
    };

    match invocation.invocation.action.as_str() {
        "message-event" => {
            if let Some(event) = &invocation.message_event {
                let entry = LogEntry {
                    kind: match event.kind {
                        MessageEventKind::Create => "create".into(),
                        MessageEventKind::Update => "update".into(),
                        MessageEventKind::Delete => "delete".into(),
                    },
                    channel_id: event.channel_id.clone(),
                    message_id: event.message_id.clone(),
                    author_id: event.author_id.clone(),
                    content: event.content.clone(),
                    timestamp_ms: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64,
                };
                state.entries.push(entry);
                if state.entries.len() > 500 {
                    state.entries.drain(..state.entries.len() - 500);
                }
            }
        }
        "show" => {
            let mut elements = vec![Element::Heading {
                text: "Message Log".into(),
            }];
            for entry in state.entries.iter().rev().take(50) {
                let author = entry.author_id.as_deref().unwrap_or("unknown");
                let content = entry
                    .content
                    .as_deref()
                    .unwrap_or("(no content)")
                    .chars()
                    .take(100)
                    .collect::<String>();
                elements.push(Element::Text {
                    text: format!("[{}] {}: {}", entry.kind, author, content),
                });
            }
            elements.push(Element::Separator);
            elements.push(Element::Text {
                text: format!("Total logged events: {}", state.entries.len()),
            });
            return AppOutput {
                output: Output {
                    panel: elements,
                    ..Default::default()
                },
                ..Default::default()
            };
        }
        "clear" => {
            state.entries.clear();
        }
        _ => {}
    }

    let storage = serde_json::to_string(&state).ok();
    AppOutput {
        output: Output { storage, ..Default::default() },
        ..Default::default()
    }
}
