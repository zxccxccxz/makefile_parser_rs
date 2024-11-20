use super::makefile::{MakefileCommandArg, MakefileRule, Rule};
use pest::iterators::Pair as PestPair;

pub(super) fn extract_variable(pair: PestPair<'_, Rule>) -> (String, String) {
    // Extract variable name & value
    let mut inner_pair = pair.into_inner();
    let (var_name, var_value) = (
        inner_pair.next().unwrap().as_str(),
        inner_pair.next().unwrap().as_str(),
    );
    (var_name.to_string(), var_value.to_string())
}

fn extract_commands(pair: PestPair<'_, Rule>) -> Vec<Vec<MakefileCommandArg>> {
    let mut res = Vec::new();
    for command in pair.into_inner() {
        let mut command_parsed = Vec::new();
        for command_arg in command.into_inner() {
            command_parsed.push(MakefileCommandArg::new(
                command_arg.as_rule(),
                command_arg.as_str().to_string(),
            ));
        }
        res.push(command_parsed);
    }
    res
}

pub(super) fn extract_rule(pair: PestPair<'_, Rule>) -> MakefileRule {
    let mut makefile_rule = MakefileRule::default();
    let mut inner_pair = pair.into_inner();

    // Target
    makefile_rule.target = inner_pair.next().unwrap().as_str().to_string();

    // Dependencies
    if let Some(inner) = inner_pair.next() {
        match inner.as_rule() {
            Rule::dependencies => {
                for dependency in inner.into_inner() {
                    makefile_rule
                        .dependencies
                        .push(dependency.as_str().to_string());
                }
            }
            Rule::commands => {
                for command in extract_commands(inner) {
                    makefile_rule.commands.push(command);
                }
                return makefile_rule;
            }
            _ => {}
        }
    }

    // Commands
    if let Some(commands) = inner_pair.next() {
        for command in extract_commands(commands) {
            makefile_rule.commands.push(command);
        }
    }

    makefile_rule
}
