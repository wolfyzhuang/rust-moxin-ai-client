use anyhow::{anyhow, Result};
use makepad_widgets::SignalToUI;
use moxin_backend::Backend;
use moxin_protocol::data::*;
use moxin_protocol::protocol::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub enum SearchAction {
    Results(Vec<Model>),
    Error,
}

#[derive(Clone)]
pub enum SearchCommand {
    Search(String),
    LoadFeaturedModels,
}

#[derive(Default, Clone)]
pub enum SearchState {
    #[default]
    Idle,
    Pending(SearchCommand, Option<SearchCommand>),
    Errored,
}
pub struct Search {
    pub keyword: Option<String>,
    pub sender: Sender<SearchAction>,
    pub receiver: Receiver<SearchAction>,
    pub state: SearchState,
}

impl Default for Search {
    fn default() -> Self {
        Search::new()
    }
}

impl Search {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let search = Self {
            keyword: None,
            sender: tx,
            receiver: rx,
            state: SearchState::Idle,
        };
        search
    }

    pub fn load_featured_models(&mut self, backend: &Backend) {
        match self.state {
            SearchState::Pending(_, ref mut next_command) => {
                *next_command = Some(SearchCommand::LoadFeaturedModels);
                return;
            }
            SearchState::Idle | SearchState::Errored => {
                self.state = SearchState::Pending(SearchCommand::LoadFeaturedModels, None);
                self.keyword = None;
            }
        }

        let (tx, rx) = channel();

        let store_search_tx = self.sender.clone();
        backend
            .command_sender
            .send(Command::GetFeaturedModels(tx))
            .unwrap();

        thread::spawn(move || {
            if let Ok(response) = rx.recv() {
                match response {
                    Ok(models) => {
                        store_search_tx.send(SearchAction::Results(models)).unwrap();
                    }
                    Err(err) => {
                        eprintln!("Error fetching models: {:?}", err);
                        store_search_tx.send(SearchAction::Error).unwrap();
                    }
                }
                SignalToUI::set_ui_signal();
            }
        });
    }

    pub fn run_or_enqueue(&mut self, keyword: String, backend: &Backend) {
        match self.state {
            SearchState::Pending(_, ref mut next