# makefile_parser_rs

## Description
Built with Rust, `makefile_parser_rs` is a library main task of which is to parse Makefiles.
It supports detecting declared variables and substituting them to commands where they are used.

## Parsing details
The parser accepts a valid Makefile and splits it as follows:
* Comments
* Variable declarations
* Rules
** Target
** Dependencies
** Commands

## Grammar
```
makefile	=  { line* }
line		=  { (variable_assignment | comment | rule)? ~ NEWLINE }
comment		=  { whitespace* ~ "#" ~ (ASCII_ALPHANUMERIC | (!NEWLINE ~ ANY))* }

variable	=  { (ASCII_ALPHANUMERIC | "_" | "." | "-" | "+" | "%" | "^" | "&")+ }
value		= @{ ("-" | "--")? ~ (string | variable)? }

variable_assignment = {
	variable
	~
	whitespace*
	~
	("=" | ":=" | "?=")
	~
	whitespace*
	~
	value
}

rule = {
	target ~ ":" ~ (" " ~ dependencies)? ~ NEWLINE ~ commands
}

target			= { (ASCII_ALPHANUMERIC | "_" | ".")+ }
dependencies	= { target ~ (" " ~ target)* }

commands			=  { (command ~ NEWLINE)* ~ command }
command_variable	= @{ "$(" ~ variable ~ ")" }
command_flag		=  { "-" ~ "-"? ~ ASCII_ALPHANUMERIC+ }
command_arg			=  { !"-" ~ variable+ }

command = {
	whitespace+
	~
	(whitespace+ | command_arg | string | command_variable | command_flag)+
}

string		= @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
whitespace	= _{ " " | "\t" | "\r" }
```

## Usage example
`makefile_parser_rs parse Makefile`
or
`make run args="parse Makefile"`

## Example Makefile parse
Given a Makefile:
```
# This is a simple Makefile

CC = gcc
CFLAGS = -Wall

main: main.cpp my_class.cpp
    clang++ -Wall -o main main.cpp my_class.cpp
    $(CC) $(CFLAGS) -o some_target dependency1 dependency2

another_target: dependency3
    @echo "Building another target"
```

Running `makefile_parser_rs parse example_make`, the output is:

```
Makefile:
|  Variables:
|  |  CC: gcc
|  |  CFLAGS: -Wall
|  Comments:
|  |  # This is a simple Makefile
|  Rule:
|  |  Target: main
|  |  Dependencies:
|  |  |  main.cpp
|  |  |  my_class.cpp
|  |  Commands:
|  |  |  clang++ -Wall -o main main.cpp my_class.cpp
|  |  |  $(CC) $(CFLAGS) -o some_target dependency1 dependency2
|  |  |  gcc -Wall -o some_target dependency1 dependency2
```