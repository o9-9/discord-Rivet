use std::collections::HashMap;

use serde::Deserialize;

use crate::api::User;

#[derive(Debug, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: Option<String>,
    pub timestamp: String,
    pub mentions: Vec<User>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PartialMessage {
    pub id: String,
    pub channel_id: String,
    pub author: Option<User>,
    pub content: Option<String>,
    pub timestamp: Option<String>,
}

impl Message {
    pub fn map_mentions(&self) -> String {
        let Some(content) = self.content.as_ref() else {
            return "(*non-text*)".to_string();
        };

        let mut ids = std::collections::HashSet::new();
        let mut temp_content = content.as_str();

        while let Some(start_idx) = temp_content.find("<@") {
            let after_prefix = &temp_content[start_idx + 2..];
            if let Some(end_idx) = after_prefix.find('>') {
                let id = &after_prefix[..end_idx];
                if id.chars().all(|c| c.is_ascii_digit()) {
                    ids.insert(id.to_string());
                }
                temp_content = &after_prefix[end_idx + 1..];
            } else {
                break;
            }
        }

        if ids.is_empty() {
            return self.content.clone().unwrap_or("(*non-text*)".to_string());
        }

        let mut mentionned_users = std::collections::HashMap::new();
        for user in self.mentions.clone() {
            mentionned_users.insert(user.id, user.global_name.unwrap_or(user.username));
        }

        let mut map_usernames: HashMap<String, String> = std::collections::HashMap::new();
        for id in ids {
            if let Some(username) = mentionned_users.get(&id) {
                map_usernames.insert(id, username.clone());
            }
        }

        let mut final_content = content.clone();
        for (id, name) in map_usernames {
            let pattern = format!("<@{id}>");
            let replacement = format!("@{name}");
            final_content = final_content.replace(&pattern, &replacement);
        }

        final_content
    }
}
