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
   