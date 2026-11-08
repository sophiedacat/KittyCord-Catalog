use kittycord_extension_sdk::{
    AppInvocation, AppOutput, Element, MessageEventKind, Output,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

kittycord_extension_sdk::export!(handle);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Command {
    trigger: String,
    response: String,
    user_count: BTreeMap<String, u64>,
    global_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct AutoReplyState {
    prefix: String,
    commands: BTreeMap<String, Command>,
}

fn handle(invocation: AppInvocation) -> AppOutput {
    let mut state: AutoReplyState = match &invocation.invocation.storage {
        Some(storage) => serde_json::from_str(storage).unwrap_or_default(),
        None => AutoReplyState::default(),
    };

    match invocation.invocation.action.as_str() {
        "message-event" => {
            if let Some(event) = &invocation.message_event {
                if event.kind == MessageEventKind::Create {
                    if let Some(content) = &event.content {
                        if content.starts_with(&state.prefix) {
                            let trigger = content[state.prefix.len()..].trim();
                            if let Some(cmd) = state.commands.get_mut(trigger) {
                                let user_id = event.author_id.clone().unwrap_or_default();
                                *cmd.user_count.entry(user_id.clone()).or_insert(0) += 1;
                                cmd.global_count += 1;

                                let mut response = cmd.response.clone();
                                let user_count = *cmd.user_count.get(&user_id).unwrap_or(&0);
                                response = response.replace(
                                    &format!("&{{user:count({trigger})}}"),
                                    &user_count.to_string()
                                );
                                response = response.replace(
                                    &format!("&{{all:count({trigger})}}"),
                                    &cmd.global_count.to_string()
                                );

                                return AppOutput {
                                    output: Output {
                                        replacement: Some(response),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                };
                            }
                        }
                    }
                }
            }
        }
        "add-command" => {
            if let Some(trigger) = invocation.invocation.values.get("trigger") {
                if let Some(response) = invocation.invocation.values.get("response") {
                    state.commands.insert(
                        trigger.clone(),
                        Command {
                            trigger: trigger.clone(),
                            response: response.clone(),
                            user_count: BTreeMap::new(),
                            global_count: 0,
                        },
                    );
                }
            }
        }
        "remove-command" => {
            if let Some(trigger) = invocation.invocation.values.get("trigger") {
                state.commands.remove(trigger);
            }
        }
        "set-prefix" => {
            if let Some(prefix) = invocation.invocation.values.get("prefix") {
                state.prefix = prefix.clone();
            }
        }
        "show" => {
            let mut elements = vec![Element::Heading {
                text: "Auto-Reply".into(),
            }];
            elements.push(Element::Text {
                text: format!("Prefix: `{}`", state.prefix),
            });
            elements.push(Element::Separator);
            for (trigger, cmd) in &state.commands {
                elements.push(Element::Text {
                    text: format!(
                        "{} → \"{}\" (user: {}, global: {})",
                        trigger,
                        cmd.response,
                        cmd.user_count.values().sum::<u64>(),
                        cmd.global_count
                    ),
                });
            }
            elements.push(Element::Separator);
            elements.push(Element::Button {
                id: "add-command".into(),
                label: "Add Command".into(),
            });
            elements.push(Element::Button {
                id: "set-prefix".into(),
                label: "Set Prefix".into(),
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
