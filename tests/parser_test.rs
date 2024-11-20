use makefile_parser_rs::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        mod comments {
            use super::*;

            #[test]
            fn valid_comment() {
                let makefile = Makefile::parse(
                    r#"
                        # First comment
                        # Hello, world!
                    "#,
                )
                .unwrap();

                let comments = makefile.get_comments();

                assert_eq!(comments.len(), 2);
                assert_eq!(comments[0].trim(), "# First comment");
                assert_eq!(comments[1].trim(), "# Hello, world!");
            }

            #[test]
            #[should_panic]
            fn invalid_comment() {
                let makefile = Makefile::parse(
                    r#"
                        First comment
                        # Hello, world!
                    "#,
                )
                .unwrap();

                let comments = makefile.get_comments();

                assert_eq!(comments.len(), 2);
            }
        }

        mod variables {
            use super::*;

            #[test]
            fn valid_variable_assignment() {
                let makefile =
                    Makefile::parse("ABC = test_var\n# simple comment\nX = 23\n").unwrap();

                let variables = makefile.get_variables();

                assert_eq!(variables.len(), 2);
                assert_eq!(variables["ABC"], "test_var".to_string());
                assert_eq!(variables["X"], "23".to_string());
            }

            #[test]
            fn invalid_variable_assignment() {
                let makefile = Makefile::parse("ABC = # cannot assign to comment\n").unwrap();

                let variables = makefile.get_variables();

                assert_eq!(variables.len(), 0);

                let makefile = Makefile::parse(
                    "ABC = :&()*9xxx\n# cannot assign invalid symbols to variable\n",
                )
                .unwrap();

                let variables = makefile.get_variables();

                assert_eq!(variables.len(), 0);
            }

            #[test]
            fn valid_variables_substitution() {
                let makefile = Makefile::parse(
                    "CC = clang++\nCFLAGS=-Wall\nmain: main.cpp my_class.cpp\n\t$(CC) $(CFLAGS) -o main main.cpp my_class.cpp\n"
                )
                .unwrap();

                let rules = makefile.get_rules();
                assert_eq!(rules.len(), 1);
                let rule = &rules[0];
                assert_eq!(rule.commands.len(), 1);
                let command = &rule.commands[0];
                assert_eq!(command.len(), 6);
                let command_as_string = &rule.commands_to_strings(false)[0];
                assert_eq!(
                    command_as_string,
                    "$(CC) $(CFLAGS) -o main main.cpp my_class.cpp"
                );
                let command_as_string = &rule.commands_to_strings(true)[0];
                assert_eq!(
                    command_as_string,
                    "clang++ -Wall -o main main.cpp my_class.cpp"
                );
            }

            #[test]
            fn invalid_variables_substitution() {
                let makefile = Makefile::parse(
                    "CC = clang++\nCFLAGS=-Wall\nmain: main.cpp my_class.cpp\n\t$(CC) $(CFLAGS -o main main.cpp my_class.cpp\n"
                )
                .unwrap();

                let rules = makefile.get_rules();
                assert_eq!(rules.len(), 0);
            }

            #[test]
            fn undeclared_variables_substitution() {
                let makefile_parse_result = Makefile::parse(
                    "main: main.cpp my_class.cpp\n\t$(CC) $(CFLAGS) -o main main.cpp my_class.cpp\n"
                );
                assert!(makefile_parse_result
                    .is_err_and(|e| e.to_string()
                        == anyhow::Error::msg("Undeclared variable `CC`").to_string()));
            }
        }

        mod rules {
            use super::*;

            #[test]
            fn valid_rule() {
                let makefile = Makefile::parse(
                    "main: main.cpp my_class.cpp\n\tclang++ -Wall -o main main.cpp my_class.cpp\n",
                )
                .unwrap();

                let rules = makefile.get_rules();
                assert_eq!(rules.len(), 1);
                let rule = &rules[0];
                assert_eq!(rule.commands.len(), 1);
                let command = &rule.commands[0];
                assert_eq!(command.len(), 6);
                let command_as_string = &rule.commands_to_strings(false)[0];
                assert_eq!(
                    command_as_string,
                    "clang++ -Wall -o main main.cpp my_class.cpp"
                );
            }

            #[test]
            fn invalid_rule() {
                // missing `\t`
                let makefile = Makefile::parse(
                    "main: main.cpp my_class.cpp\nclang++ -Wall -o main main.cpp my_class.cpp\n",
                )
                .unwrap();

                let rules = makefile.get_rules();

                assert_eq!(rules.len(), 0);
                // invalid `()`
                let makefile = Makefile::parse(
                    "main: ma()in.cpp my_class.cpp\nclang++ -Wall -o main main.cpp my_class.cpp\n",
                )
                .unwrap();

                let rules = makefile.get_rules();
                assert_eq!(rules.len(), 0);
            }

            #[test]
            fn valid_rule_few_commands() {
                let makefile = Makefile::parse(
                    "main: main.cpp my_class.cpp\n\tclang++ -Wall -o main main.cpp my_class.cpp\n\techo \"Building...\"\n"
                )
                .unwrap();

                let rules = makefile.get_rules();
                assert_eq!(rules.len(), 1);
                let rule = &rules[0];
                assert_eq!(rule.commands.len(), 2);
                let command = &rule.commands[0];
                assert_eq!(command.len(), 6);
                let command_as_string = &rule.commands_to_strings(false)[0];
                assert_eq!(
                    command_as_string,
                    "clang++ -Wall -o main main.cpp my_class.cpp"
                );
            }

            #[test]
            fn invalid_rule_few_commands() {
                let makefile = Makefile::parse(
                    "main: main.cpp my_class.cpp\n\tclang++ -Wall -o main main.cpp my_class.cpp echo \"Building...\"\n"
                )
                .unwrap();

                let rules = makefile.get_rules();
                assert_eq!(rules.len(), 1);
                let rule = &rules[0];
                assert_eq!(rule.commands.len(), 1);
                let command = &rule.commands[0];
                assert_ne!(command.len(), 6);
                let command_as_string = &rule.commands_to_strings(false)[0];
                assert_ne!(
                    command_as_string,
                    "clang++ -Wall -o main main.cpp my_class.cpp"
                );
            }
        }
    }
}
