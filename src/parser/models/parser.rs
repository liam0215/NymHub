use crate::command::Action;
use crate::command::Command;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/models/dsl.pest"]
pub struct NymHubDSL;

type PestError = Box<pest::error::Error<Rule>>;

impl NymHubDSL {
    pub fn parse_msg(msg: &str) -> Result<Vec<Command>, PestError> {
        let pairs = NymHubDSL::parse(Rule::main, msg)?;
        let mut commands: Vec<Command> = Vec::new();
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::main => {
                    for stmt in pair.into_inner() {
                        if stmt.as_rule() == Rule::stmt {
                            let mut speaker = "";
                            for action in stmt.into_inner() {
                                match action.as_rule() {
                                    Rule::speaker => {
                                        speaker =
                                            action.into_inner().next().unwrap().as_span().as_str();
                                    }
                                    Rule::says => {
                                        commands
                                            .push(Command::new(speaker.to_string(), Action::Says));
                                    }
                                    Rule::assign => {
                                        commands.push(Command::new(
                                            speaker.to_string(),
                                            Action::Assign,
                                        ));
                                    }
                                    Rule::create => {
                                        commands.push(Command::new(
                                            speaker.to_string(),
                                            Action::Create,
                                        ));
                                    }
                                    Rule::speaksfor => {
                                        commands.push(Command::new(
                                            speaker.to_string(),
                                            Action::Speaksfor,
                                        ));
                                    }
                                    Rule::subprincipal => {
                                        commands.push(Command::new(
                                            speaker.to_string(),
                                            Action::Subprincipal,
                                        ));
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        };
                    }
                }
                _ => unreachable!(),
            };
        }
        Ok(commands)
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
        assert_eq!(commands[0].speaker, "user1");
        assert_eq!(commands[0].action, Action::Create);
    }
}
