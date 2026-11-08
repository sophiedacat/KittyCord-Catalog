use kittycord_extension_sdk::{
    AppInvocation, AppOutput, Element, Output,
};
use serde::{Deserialize, Serialize};

kittycord_extension_sdk::export!(handle);

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct BulkDeleteState {
    selected_channels: Vec<String>,
    selected_categories: Vec<String>,
}

fn handle(invocation: AppInvocation) -> AppOutput {
    let mut state: BulkDeleteState = match &invocation.invocation.storage {
        Some(storage) => serde_json::from_str(storage).unwrap_or_default(),
        None => BulkDeleteState::default(),
    };

    match invocation.invocation.action.as_str() {
        "toggle-channel" => {
            if let Some(channel_id) = invocation.invocation.values.get("channel_id") {
                if state.selected_channels.iter().any(|id| id == channel_id) {
                    state.selected_channels.retain(|id| id != channel_id);
                } else {
                    state.selected_channels.push(channel_id.clone());
                }
            }
        }
        "toggle-category" => {
            if let Some(category_id) = invocation.invocation.values.get("category_id") {
                if state.selected_categories.iter().any(|id| id == category_id) {
                    state.selected_categories.retain(|id| id != category_id);
                } else {
                    state.selected_categories.push(category_id.clone());
                }
            }
        }
        "delete-selected" => {
            // Host would handle actual deletion
        }
        "show" => {
            let mut elements = vec![Element::Heading {
                text: "Bulk Channel Delete".into(),
            }];
            elements.push(Element::Text {
                text: "Select channels and categories to delete in bulk.".into(),
            });
            elements.push(Element::Separator);
            elements.push(Element::Text {
                text: format!("Selected channels: {}", state.selected_channels.len()),
            });
            elements.push(Element::Text {
                text: format!("Selected categories: {}", state.selected_categories.len()),
            });
            elements.push(Element::Separator);
            elements.push(Element::Button {
                id: "delete-selected".into(),
                label: "Delete Selected".into(),
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
