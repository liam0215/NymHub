#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub speaker : String,
    pub action : Action,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Create,
    Assign,
    Says,
    Speaksfor,
    Subprincipal,
}

impl Command {
    pub fn new(speaker: String, action: Action) -> Command {
        Command {
            speaker,
            action,
        }
    }
}