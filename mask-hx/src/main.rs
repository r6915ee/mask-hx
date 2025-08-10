//! A program made to handle [Haxe](https://haxe.org) versions.
//!
//! `mask-hx` aims to simplify [Haxe](https://haxe.org) version
//! management. [Haxe](https://haxe.org), unlike most other toolchains,
//! does not play well with projects as a result of its
//! versioning mechanism including syntax changes. `mask-hx`
//! aims to simplify the process of version management with
//! [Haxe](https://haxe.org).

use std::process;

use clap::{ArgAction, ArgMatches, Command, arg, command, crate_name, value_parser};

use libmask::{config, fetcher};

/// Defines the "output level" of various functions.
///
/// [OutputLevel] is useful to define how the program should
/// print to the standard output. It is ignored in some cases.
#[derive(Clone)]
enum OutputLevel {
    /// Only the bare minimum will be printed.
    Quiet,
    /// Some printing will be performed. However, it doesn't expose certain information.
    Normal,
    /// Print everything that is thrown.
    Verbose,
}

/// Print to the standard output.
///
/// This macro functions identically to the [println] macro, except
/// it compares a required [OutputLevel] and the current [OutputLevel] to
/// see if the latter is greater or equal to the required output level,
/// and only printing if this comparison succeeds.
///
/// Additionally, `text` can be an expression. This is useful for concatenation
/// reasons, but more importantly, that means that the [format] macro can
/// be used as the value.
///
/// # Examples
///
/// ```
/// let current_level: OutputLevel = OutputLevel::Normal;
///
/// print_to_stdout!(OutputLevel::Normal, current_level, "The current output level is greater than the required output level");
/// ```
macro_rules! print_to_stdout {
    ($required_level: expr, $current_level: expr, $text: literal) => {
        if $current_level as u8 >= $required_level as u8 {
            println!("{}", $text);
        }
    };

    ($required_level: expr, $current_level: expr, $text: expr) => {
        if $current_level as u8 >= $required_level as u8 {
            println!("{}", $text);
        }
    };
}

/// Defines the final output of `mask-hx`.
struct CommandResult {
    /// The message to print when `mask-hx` finishes.
    message: String,
    /// The exit status code of `mask-hx`.
    code: i32,
}

fn handle_commands() -> ArgMatches {
    command!()
        .arg(arg!(-q --quiet "Enable quiet output"))
        .arg(arg!(-v --verbose "Enable verbose output"))
        .subcommand(
            Command::new("check")
                .about("Checks whether or not a Haxe version is installed")
                .long_about(
                    "This checks the validity of a Haxe installation. \
                    Specifically, it checks for the existence of a folder in the \
                    ~/.haxe/ directory, where ~ is the home directory, and checks \
                    if the standard library is present as well.\n\n\
                    If you don't specify a Haxe version, then the .mask configuration \
                    will be read.",
                )
                .arg(
                    arg!([HAXE_VERSION] "Haxe version to check, if no configuration is specified"),
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
        .get_matches()
}

/// The entry point of the program.
///
/// This handles the arguments, as well as how the program should exit.
fn main() {
    let matches: ArgMatches = handle_commands();

    let output_level: OutputLevel =
        match matches.get_flag("verbose") as i8 - matches.get_flag("quiet") as i8 {
            -1 => OutputLevel::Quiet,
            0 => OutputLevel::Normal,
            1 => OutputLevel::Verbose,
            _ => OutputLevel::Quiet,
        };

    let result: CommandResult;

    if let Some(matches) = matches.subcommand_matches("check") {
        fn get_result(check: Result<bool, std::io::Error>) -> CommandResult {
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
        if let Some(version) = matches.get_one::<String>("HAXE_VERSION") {
            result = get_result(fetcher::is_haxe_version_installed(version));
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
