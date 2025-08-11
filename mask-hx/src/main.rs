//! A program made to handle [Haxe](https://haxe.org) versions.
//!
//! `mask-hx` aims to simplify [Haxe](https://haxe.org) version
//! management. [Haxe](https://haxe.org), unlike most other toolchains,
//! does not play well with projects as a result of its
//! versioning mechanism including syntax changes. `mask-hx`
//! aims to simplify the process of version management with
//! [Haxe](https://haxe.org).

use std::{io::Error, process};

use clap::{ArgAction, ArgMatches, Command, arg, command};

use libmask::{config, fetcher, interactive};

/// Defines the final output of `mask-hx`.
struct CommandResult {
    /// The message to print when `mask-hx` finishes.
    message: String,
    /// The exit status code of `mask-hx`.
    code: i32,
}

/// Give possible commands to [clap].
fn handle_commands() -> ArgMatches {
    command!()
        .arg(
            arg!(-e --explicit "Use an explicit Haxe version")
                .action(ArgAction::Set)
                .value_name("HAXE_VERSION"),
        )
        .subcommand(
            Command::new("check")
                .about("Checks whether or not a Haxe version is installed")
                .long_about(
                    "This checks the validity of a Haxe installation. \
                    Specifically, it checks for the existence of a folder in the \
                    ~/.haxe/ directory, where ~ is the home directory, and checks \
                    if the standard library is present as well.\n\n\
                    If the explicit argument isn't used, then the .mask configuration \
                    will be read.",
                ),
        )
        .subcommand(
            Command::new("switch")
                .about("Changes the configuration to use a different, valid Haxe version")
                .long_about(
                    "This initially checks the validity of a Haxe installation, \
                    and then switches the configuration to use that specified Haxe \
                    version.",
                )
                .arg(arg!(<HAXE_VERSION> "The Haxe version to switch to")),
        )
        .subcommand(
            Command::new("exec")
                .about("Executes the Haxe compiler")
                .long_about(
                    "This checks for the existence of the Haxe compiler, and then \
                    executes it. The Haxe compiler used is the one provided by the \
                    currently configured version.",
                )
                .disable_help_flag(true)
                .arg(
                    arg!(<ARGUMENTS>... "Specify the arguments to pass to the compiler")
                        .value_delimiter(' ')
                        .allow_hyphen_values(true)
                        .trailing_var_arg(true),
                ),
        )
        .get_matches()
}

/// The entry point of the program.
///
/// This handles the arguments, as well as how the program should exit.
fn main() {
    let matches: ArgMatches = handle_commands();

    let result: CommandResult;
    let mut haxe_version: Option<String> = None;

    if let Some(version) = matches.get_one::<String>("explicit") {
        haxe_version = Some(version.to_string());
    }

    if let Some(_) = matches.subcommand_matches("check") {
        fn get_result(check: Result<bool, Error>) -> CommandResult {
            match check {
                Ok(bool_opt) => match bool_opt {
                    true => CommandResult {
                        message: String::from("Haxe version specified is usable"),
                        code: 0,
                    },
                    false => CommandResult {
                        message: String::from(
                            "Haxe version used either lacks a standard library or cannot be found",
                        ),
                        code: 1,
                    },
                },
                Err(e) => CommandResult {
                    message: format!("io error: {}", e),
                    code: 1,
                },
            }
        }
        if haxe_version.is_some() {
            result = get_result(fetcher::is_haxe_version_installed(
                haxe_version.unwrap().as_str(),
            ));
        } else {
            result = get_result(fetcher::is_config_version_installed());
        }
    } else if let Some(matches) = matches.subcommand_matches("switch") {
        result = match fetcher::is_haxe_version_installed(
            matches.get_one::<String>("HAXE_VERSION").unwrap().as_str(),
        ) {
            Ok(bool_opt) => match bool_opt {
                true => {
                    match config::write(matches.get_one::<String>("HAXE_VERSION").unwrap().clone())
                    {
                        Ok(_) => CommandResult {
                            message: String::from("successfully switched Haxe version"),
                            code: 0,
                        },
                        Err(e) => CommandResult {
                            message: format!("io error: {}", e),
                            code: 1,
                        },
                    }
                }
                false => CommandResult {
                    message: String::from("Haxe version specified is not valid"),
                    code: 1,
                },
            },
            Err(e) => CommandResult {
                message: format!("io error: {}", e),
                code: 1,
            },
        };
    } else if let Some(matches) = matches.subcommand_matches("exec") {
        let is_version_installed: Result<bool, Error>;

        if haxe_version.is_some() {
            is_version_installed =
                fetcher::is_haxe_version_installed(haxe_version.as_ref().unwrap().as_str())
        } else {
            is_version_installed = fetcher::is_config_version_installed();
        }

        result = match is_version_installed {
            Ok(bool_opt) => match bool_opt {
                true => {
                    let args = matches.get_many::<String>("ARGUMENTS").unwrap();
                    let mut args_vec: Vec<String> = vec![];

                    for arg in args {
                        args_vec.push(arg.to_string());
                    }

                    match interactive::haxe(args_vec, haxe_version) {
                        Ok(status) => match status.code() {
                            Some(code) => match code {
                                0 => CommandResult {
                                    message: String::from("successfully ran Haxe"),
                                    code: 0,
                                },
                                _ => CommandResult {
                                    message: String::from(
                                        "successfully ran Haxe, but error was returned",
                                    ),
                                    code: code,
                                },
                            },
                            None => CommandResult {
                                message: String::from(
                                    "successfully ran Haxe, but was terminated by a signal",
                                ),
                                code: 1,
                            },
                        },
                        Err(e) => CommandResult {
                            message: format!("io error: {}", e),
                            code: 1,
                        },
                    }
                }
                false => {
                    if haxe_version.clone().is_some() {
                        CommandResult {
                            message: format!(
                                "Haxe version {} is not installed",
                                haxe_version.unwrap().as_str()
                            ),
                            code: 1,
                        }
                    } else {
                        CommandResult {
                            message: format!(
                                "Haxe version {} is not installed",
                                config::read().unwrap()
                            ),
                            code: 1,
                        }
                    }
                }
            },
            Err(e) => CommandResult {
                message: format!("io error: {}", e),
                code: 1,
            },
        }
    } else {
        result = CommandResult {
            message: String::from(
                "invalid subcommand, or no subcommand was passed; use 'mask-hx help' for a list of commands",
            ),
            code: 22,
        }
    }

    println!("mask: {}", result.message);

    process::exit(result.code);
}
