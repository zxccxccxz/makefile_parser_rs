use anyhow::Result;
use std::collections::HashMap;

use super::*;

use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "./make.pest"]
struct MakefileParser;

#[derive(Default)]
pub struct MakefileRule {
    pub target: String,
    pub dependencies: Vec<String>,
    pub commands: Vec<Vec<MakefileCommandArg>>,
}
pub enum MakefileCommandArg {
    Argument(String),
    Flag(String),
    Variable { name: String, value: String },
}

pub struct Makefile {
    pub(super) variables: HashMap<String, String>,
    pub(super) comments: Vec<String>,
    pub(super) rules: Vec<MakefileRule>,
}

impl MakefileCommandArg {
    pub(super) fn new(arg_type: Rule, raw_value: String) -> Self {
        match arg_type {
            Rule::command_variable => Self::Variable {
                name: raw_value[2..raw_value.len() - 1].to_string(),
                value: "UNKNOWN".to_string(),
            },
            Rule::command_flag => Self::Flag(raw_value),
            _ => Self::Argument(raw_value),
        }
    }
}

impl MakefileRule {
    pub fn commands_to_strings(&self, substitute_variables: bool) -> Vec<String> {
        self.commands
            .iter()
            .map(|args| {
                args.iter()
                    .map(|arg| {
                        let arg_as_string = match arg {
                            MakefileCommandArg::Argument(s) => s.to_string(),
                            MakefileCommandArg::Flag(s) => s.to_string(),
                            MakefileCommandArg::Variable { name, value } => {
                                if substitute_variables {
                                    value.to_string()
                                } else {
                                    format!("$({name})")
                                }
                            }
                        };
                        arg_as_string + " "
                    })
                    .collect::<String>()
            })
            .map(|s| s.trim().to_string())
            .collect()
    }
}

impl Makefile {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            comments: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn get_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    pub fn get_comments(&self) -> &Vec<String> {
        &self.comments
    }

    pub fn get_rules(&self) -> &Vec<MakefileRule> {
        &self.rules
    }

    fn fill_variables(&mut self) -> Result<()> {
        for rule in &mut self.rules {
            for command in &mut rule.commands {
                for command_arg in command {
                    if let MakefileCommandArg::Variable { name, value } = command_arg {
                        if name == "args" {
                            *value = "USER_DEFINED".to_string();
                            continue;
                        }
                        *value = self
                            .variables
                            .get(&name.clone())
                            .ok_or(anyhow::Error::msg(format!(
                                "Undeclared variable `{}`",
                                &name
                            )))?
                            .clone();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn parse_file(filepath: &str) -> Result<Self> {
        let file = std::fs::read_to_string(filepath)?;
        Self::parse(&file)
    }

    pub fn parse(input: &str) -> Result<Self> {
        let mut makefile = Self::new();
        let pairs = MakefileParser::parse(Rule::makefile, input)?;

        for pair in pairs.flatten() {
            match pair.as_rule() {
                Rule::comment => {
                    makefile.comments.push(pair.as_str().to_string());
                }
                Rule::variable_assignment => {
                    let (var_name, var_value) = parse_utils::extract_variable(pair);
                    makefile.variables.insert(var_name, var_value);
                }
                Rule::rule => {
                    let rule = parse_utils::extract_rule(pair);
                    makefile.rules.push(rule);
                }
                _ => {}
            };
        }

        makefile.fill_variables()?;

        Ok(makefile)
    }
}
