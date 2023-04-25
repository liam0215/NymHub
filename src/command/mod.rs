#![allow(dead_code)]

pub trait Command {
    fn execute(&self);
    fn speaker(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DSLType {
    #[default]
    Bool,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DSLValue {
    Bool(bool),
    Float(f32),
}

impl Default for DSLValue {
    fn default() -> Self {
        DSLValue::Bool(false)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Create {
    pub speaker: String,
    pub id: String,
    pub v_type: DSLType,
    pub free: bool,
}

impl Create {
    pub fn new(speaker: String, id: String, v_type: DSLType, free: bool) -> Self {
        Create {
            speaker,
            id,
            v_type,
            free,
        }
    }

    pub fn with_speaker(speaker: String) -> Self {
        Create {
            speaker,
            id: String::new(),
            v_type: DSLType::Bool,
            free: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Assign {
    pub speaker: String,
    pub id: String,
    pub value: DSLValue,
}

impl Assign {
    pub fn new(speaker: String, id: String, value: DSLValue) -> Self {
        Assign { speaker, id, value }
    }

    pub fn with_speaker(speaker: String) -> Self {
        Assign {
            speaker,
            id: String::new(),
            value: DSLValue::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Says {
    pub speaker: String,
    // TODO: Add real bool expression
    pub bool_exprs: Vec<bool>,
}

impl Says {
    pub fn new(speaker: String, bool_exprs: Vec<bool>) -> Self {
        Says {
            speaker,
            bool_exprs,
        }
    }

    pub fn with_speaker(speaker: String) -> Self {
        Says {
            speaker,
            bool_exprs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Speaksfor {
    pub speaker: String,
    pub principal1: String,
    pub principal2: String,
    pub id: String,
}

impl Speaksfor {
    pub fn new(speaker: String, principal1: String, principal2: String, id: String) -> Self {
        Speaksfor {
            speaker,
            principal1,
            principal2,
            id,
        }
    }

    pub fn with_speaker(speaker: String) -> Self {
        Speaksfor {
            speaker,
            principal1: String::new(),
            principal2: String::new(),
            id: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Subprincipal {
    pub speaker: String,
    pub principal1: String,
    pub principal2: String,
}

impl Subprincipal {
    pub fn new(speaker: String, principal1: String, principal2: String) -> Self {
        Subprincipal {
            speaker,
            principal1,
            principal2,
        }
    }

    pub fn with_speaker(speaker: String) -> Self {
        Subprincipal {
            speaker,
            principal1: String::new(),
            principal2: String::new(),
        }
    }
}

// TODO: Fill in execute with actual operations
impl Command for Create {
    fn execute(&self) {
        println!("Create: {:?}", self);
    }

    fn speaker(&self) -> &str {
        &self.speaker
    }
}

impl Command for Assign {
    fn execute(&self) {
        println!("Assign: {:?}", self);
    }

    fn speaker(&self) -> &str {
        &self.speaker
    }
}

impl Command for Says {
    fn execute(&self) {
        println!("Says: {:?}", self);
    }

    fn speaker(&self) -> &str {
        &self.speaker
    }
}

impl Command for Speaksfor {
    fn execute(&self) {
        println!("Speaksfor: {:?}", self);
    }

    fn speaker(&self) -> &str {
        &self.speaker
    }
}

impl Command for Subprincipal {
    fn execute(&self) {
        println!("Subprincipal: {:?}", self);
    }

    fn speaker(&self) -> &str {
        &self.speaker
    }
}
