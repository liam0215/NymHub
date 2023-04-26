#![allow(dead_code)]

pub trait Command {
    fn execute(&self);
    fn speaker(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub enum DSLType {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Create {
    pub speaker: String,
    pub id: String,
    pub dsl_type: DSLType,
    pub free: BoolExpr,
}

impl Create {
    pub fn new(speaker: String, id: String, dsl_type: DSLType, free: BoolExpr) -> Self {
        Create {
            speaker,
            id,
            dsl_type,
            free,
        }
    }

    pub fn with_speaker(speaker: String) -> Self {
        Create {
            speaker,
            id: String::new(),
            dsl_type: DSLType::Bool,
            free: BoolExpr::Bool(false),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    And,
    Or,
    Equals,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolExpr {
    Bool(bool),
    Op(BoolOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimExpr {
    Id(String),
    Value(DSLValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithExpr {
    Number(f32),
    Op(ArithOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Prim(PrimExpr),
    Bool(BoolExpr),
    Arithmetic(ArithExpr),
}
