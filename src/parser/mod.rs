#![allow(dead_code)]

use crate::command::{Assign, Command, Create, Says, Speaksfor, Subprincipal};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/dsl.pest"]
pub struct NymHubDSL;

type PestError = Box<pest::error::Error<Rule>>;

impl NymHubDSL {
    pub fn parse_msg(msg: &str) -> Result<Vec<Box<dyn Command>>, PestError> {
        dbg!(msg);
        let pairs = NymHubDSL::parse(Rule::main, msg)?;
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        for pair in pairs.into_iter() {
            if pair.as_rule() == Rule::main {
                Self::parse_main(pair, &mut commands)?;
            } else {
                unreachable!();
            }
        }
        Ok(commands)
    }

    fn parse_main(main: Pair<Rule>, commands: &mut Vec<Box<dyn Command>>) -> Result<(), PestError> {
        for stmt in main.into_inner() {
            let stmt_rule = stmt.as_rule();
            if stmt_rule == Rule::stmt {
                Self::parse_stmt(stmt, commands)?;
            } else if stmt_rule != Rule::EOI && stmt_rule != Rule::WHITESPACE {
                unreachable!();
            }
        }
        Ok(())
    }

    fn parse_stmt(stmt: Pair<Rule>, commands: &mut Vec<Box<dyn Command>>) -> Result<(), PestError> {
        let mut speaker = "";
        for action in stmt.into_inner() {
            if action.as_rule() == Rule::speaker {
                speaker = action.into_inner().next().unwrap().as_span().as_str();
            } else {
                Self::parse_action(&speaker, &action, commands)?;
            }
        }
        Ok(())
    }

    fn parse_action(
        speaker: &&str,
        action: &Pair<Rule>,
        commands: &mut Vec<Box<dyn Command>>,
    ) -> Result<(), PestError> {
        match action.as_rule() {
            Rule::says => {
                commands.push(Box::new(Says::with_speaker(speaker.to_string())));
            }
            Rule::assign => {
                commands.push(Box::new(Assign::with_speaker(speaker.to_string())));
            }
            Rule::create => {
                commands.push(Box::new(Create::with_speaker(speaker.to_string())));
            }
            Rule::speaksfor => {
                commands.push(Box::new(Speaksfor::with_speaker(speaker.to_string())));
            }
            Rule::subprincipal => {
                commands.push(Box::new(Subprincipal::with_speaker(speaker.to_string())));
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_parse_create() {
        const MSG: &str = "user1 create ac_on bool true\n";
        let _pairs = NymHubDSL::parse(Rule::main, MSG).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn test_parse_says() {
        const MSG: &str = "user1 says ( or ( < temp 70 ) ac_on )\n";
        let _pairs = NymHubDSL::parse(Rule::main, MSG).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn test_parse_assign() {
        const MSG: &str = "thermostat assign temp 72\n";
        let _pairs = NymHubDSL::parse(Rule::main, MSG).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn test_gibberish_fails() {
        const MSG: &str = "asdifuashdfalakjsdhflkh\n";
        assert!(NymHubDSL::parse(Rule::main, MSG).is_err());
    }

    #[test]
    fn test_parse_msg_create() {
        const MSG: &str = "user1 create ac_on bool true\n";
        let commands = NymHubDSL::parse_msg(&MSG.to_string()).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].speaker(), "user1");
    }
}
