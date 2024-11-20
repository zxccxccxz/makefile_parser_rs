use super::makefile::{Makefile, MakefileCommandArg};

impl std::fmt::Display for Makefile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Makefile:")?;
        // Variables
        if self.variables.len() > 0 {
            writeln!(f, "|  Variables:")?;
        }
        for (name, value) in &self.variables {
            writeln!(f, "|  |  {name}: {value}")?;
        }
        // Comments
        if self.comments.len() > 0 {
            writeln!(f, "|  Comments:")?;
        }
        for comment in &self.comments {
            writeln!(f, "|  |  {comment}")?;
        }
        // Rules
        for rule in &self.rules {
            writeln!(f, "|  Rule:")?;
            writeln!(f, "|  |  Target: {}", rule.target)?;
            if rule.dependencies.len() > 0 {
                writeln!(f, "|  |  Dependencies:")?;
            }
            for dependency in &rule.dependencies {
                writeln!(f, "|  |  |  {dependency}")?;
            }
            if rule.commands.len() > 0 {
                writeln!(f, "|  |  Commands:")?;
            }
            for command in &rule.commands {
                write!(f, "|  |  |  ")?;
                let mut should_print_variables = false;
                for command_arg in command {
                    match command_arg {
                        MakefileCommandArg::Argument(arg) => write!(f, "{arg} ")?,
                        MakefileCommandArg::Flag(flag) => write!(f, "{flag} ")?,
                        MakefileCommandArg::Variable { name, value: _ } => {
                            should_print_variables = true;
                            write!(f, "$({name}) ")?
                        }
                    };
                }
                if should_print_variables {
                    write!(f, "\n|  |  |  ")?;
                    for command_arg in command {
                        match command_arg {
                            MakefileCommandArg::Argument(arg) => write!(f, "{arg} ")?,
                            MakefileCommandArg::Flag(flag) => write!(f, "{flag} ")?,
                            MakefileCommandArg::Variable { name: _, value } => {
                                write!(f, "{value} ")?
                            }
                        };
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
