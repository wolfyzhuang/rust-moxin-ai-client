
use makepad_widgets::SignalToUI;
use moxin_backend::Backend;
use moxin_protocol::data::FileID;
use moxin_protocol::open_ai::*;
use moxin_protocol::protocol::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub type ChatID = u128;

#[derive(Clone, Debug)]
pub enum ChatTokenArrivalAction {
    AppendDelta(String),
    StreamingDone,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub id: usize,
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn is_assistant(&self) -> bool {
        matches!(self.role, Role::Assistant)
    }
}

#[derive(Debug, Default)]
enum TitleState {
    #[default]
    Default,
    Updated,
}

#[derive(Debug)]
pub struct Chat {
    /// Unix timestamp in ms.
    pub id: ChatID,
    pub model_filename: String,
    pub file_id: FileID,
    pub messages: Vec<ChatMessage>,
    pub messages_update_sender: Sender<ChatTokenArrivalAction>,
    pub messages_update_receiver: Receiver<ChatTokenArrivalAction>,
    pub is_streaming: bool,

    title: String,
    /// Know when title was updated by user.
    title_state: TitleState,
}

impl Chat {
    pub fn new(filename: String, file_id: FileID) -> Self {
        let (tx, rx) = channel();

        // Get Unix timestamp in ms for id.
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Couldn't get Unix timestamp, time went backwards")
            .as_millis();

        Self {
            id,
            title: String::from("New Chat"),
            model_filename: filename,
            file_id,
            messages: vec![],
            messages_update_sender: tx,
            messages_update_receiver: rx,
            is_streaming: false,
            title_state: TitleState::default(),
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.title_state = TitleState::Updated;
    }

    // TODO: this feels kinda wrong.
    fn update_title_based_on_first_message(&mut self) {
        // If it hasnt been updated, and theres at least one message, use the first
        // one as title. Else we just return the default one.
        if matches!(self.title_state, TitleState::Default) {
            if let Some(message) = self.messages.first() {
                let max_char_length = 25;
                let ellipsis = "...";

                let title = if message.content.len() > max_char_length {
                    let mut truncated = message
                        .content
                        .chars()
                        .take(max_char_length)
                        .collect::<String>()
                        .replace('\n', " ");
                    truncated.push_str(ellipsis);
                    truncated
                } else {
                    message.content.clone()
                };

                self.set_title(title);
            }
        }
    }
    pub fn send_message_to_model(&mut self, prompt: String, backend: &Backend) {
        let (tx, rx) = channel();
        let mut messages: Vec<_> = self
            .messages
            .iter()
            .map(|message| Message {
                content: message.content.clone(),
                role: message.role.clone(),
                name: None,
            })
            .collect();

        messages.push(Message {
            content: prompt.clone(),
            role: Role::User,
            name: None,
        });

        let cmd = Command::Chat(
            ChatRequestData {
                messages,
                model: "llama-2-7b-chat.Q5_K_M".to_string(),
                frequency_penalty: None,
                logprobs: None,
                top_logprobs: None,
                max_tokens: None,
                presence_penalty: None,
                seed: None,
                stop: None,
                stream: Some(true),
                temperature: None,
                top_p: None,
                n: None,
                logit_bias: None,
            },
            tx,
        );

        let next_id = self.messages.last().map(|m| m.id).unwrap_or(0) + 1;
        self.messages.push(ChatMessage {
            id: next_id,
            role: Role::User,
            content: prompt.clone(),
        });
        self.messages.push(ChatMessage {
            id: next_id + 1,
            role: Role::Assistant,
            content: "".to_string(),
        });
