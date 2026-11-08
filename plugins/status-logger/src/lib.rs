use kittycord_extension_sdk::{
	AppInvocation, AppOutput, Element, Output,
};
use serde::{Deserialize, Serialize};

kittycord_extension_sdk::export!(handle);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusLogEntry {
	user_id: String,
	old_status: Option<String>,
	new_status: Option<String>,
	timestamp_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct StatusLoggerState {
	tracked_users: Vec<String>,
	entries: Vec<StatusLogEntry>,
}

fn handle(invocation: AppInvocation) -> AppOutput {
	let mut state: StatusLoggerState = match &invocation.invocation.storage {
		Some(storage) => serde_json::from_str(storage).unwrap_or_default(),
		None => StatusLoggerState::default(),
	};

	match invocation.invocation.action.as_str() {
		"presence-event" => {
			if let Some(snapshot) = &invocation.app {
				if let Some(presence) = &snapshot.presence {
					for item in &presence.items {
						let should_track = state.tracked_users.is_empty()
							|| state.tracked_users.iter().any(|id| id == &item.user_id);
						if should_track {
							let old_entry = state.entries.iter().rev().find(|e| e.user_id == item.user_id);
							let old_status = old_entry.and_then(|e| e.new_status.clone());
							let entry = StatusLogEntry {
								user_id: item.user_id.clone(),
								old_status,
								new_status: Some(item.status.clone()),
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
				}
			}
		}
		"track" => {
			if let Some(user_id) = invocation.invocation.values.get("user_id") {
				if !state.tracked_users.iter().any(|id| id == user_id) {
					state.tracked_users.push(user_id.clone());
				}
			}
		}
		"untrack" => {
			if let Some(user_id) = invocation.invocation.values.get("user_id") {
				state.tracked_users.retain(|id| id != user_id);
			}
		}
		"show" => {
			let mut elements = vec![Element::Heading {
				text: "Status Log".into(),
			}];
			elements.push(Element::Text {
				text: format!(
					"Tracking {} users",
					if state.tracked_users.is_empty() {
						"all friends".into()
					} else {
						format!("{} specific users", state.tracked_users.len())
					}
				),
			});
			elements.push(Element::Separator);
			for entry in state.entries.iter().rev().take(50) {
				let old = entry.old_status.as_deref().unwrap_or("unknown");
				let new = entry.new_status.as_deref().unwrap_or("unknown");
				elements.push(Element::Text {
					text: format!("{} {} → {}", entry.user_id, old, new),
				});
			}
			elements.push(Element::Separator);
			elements.push(Element::Button {
				id: "track-all".into(),
				label: "Track all friends".into(),
			});
			elements.push(Element::Button {
				id: "track-none".into(),
				label: "Track no one".into(),
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
