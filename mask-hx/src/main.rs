//! A program made to handle [Haxe](https://haxe.org) versions.
//!
//! `mask-hx` aims to simplify [Haxe](https://haxe.org) version
//! management. [Haxe](https://haxe.org), unlike most other toolchains,
//! does not play well with projects as a result of its
//! versioning mechanism including syntax changes. `mask-hx`
//! aims to simplify the process of version management with
//! [Haxe](https://haxe.org).

use std::{env, io::Error, path::PathBuf, process};

use clap::{ArgAction, ArgMatches, Command, arg, command, parser::ValuesRef};

use libmask::{config::Config, fetcher::HaxeVersion, interactive};

/// Defines the final output of `mask-hx`.
struct CommandResult {
    /// The message to print when `mask-hx` finishes.
    message: String,
    /// The exit status code of `mask-hx`.
    code: i32,
    /// Display the exit message, even if the exit status code is 0.
    force: bool,
}

/// Give possible commands to [clap].
fn handle_commands() -> ArgMatches {
    command!()
        .arg(
            arg!(-e --explicit "Use an explicit Haxe version")
                .action(ArgAction::Set)
                .value_name("HAXE_VERSION"),
        )
        .arg(
            arg!(-c --config "Specify a configuration file")
                .action(ArgAction::Set)
                .value_name("CONFIG"),
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
        .subcommand(
            Command::new("lib")
                .about("Executes Haxelib")
                .long_about(
                    "This acts similar to the exec subcommand, but instead performs \
                    operations on Haxelib, the Haxe package manager.",
                )
                .disable_help_flag(true)
                .arg(
                    arg!(<ARGUMENTS>... "Specify the arguments to pass to Haxelib")
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
    let config: Config;

    if let Some(version) = matches.get_one::<String>("explicit") {
        config = Config(HaxeVersion(version.clone()));
    } else if let Some(version) = matches.get_one::<String>("config") {
        config = Config::new(Some(version)).unwrap_or(Config::default());
    } else {
        if let Ok(data) = env::var("MASK_VERSION") {
            config = Config(HaxeVersion(data));
        } else {
            config = Config::default();
        }
    }

    if let Some(_) = matches.subcommand_matches("check") {
        result = match config.0.get_path_installed() {
            Ok(data) => {
                todo!()
            }
            Err(e) => CommandResult {
                message: e.to_string(),
                code: e.kind() as i32,
                force: false,
            },
        };
    } else if let Some(matches) = matches.subcommand_matches("switch") {
    } else if let Some(matches) = matches.subcommand_matches("exec") {
    } else if let Some(matches) = matches.subcommand_matches("lib") {
    } else {
        result = CommandResult {
            message: String::from(
                "invalid subcommand, or no subcommand was passed; use 'mask-hx help' for a list of commands",
            ),
            code: 22,
            force: false,
        }
    }

    if result.code != 0 || result.force {
        println!("mask: {}", result.message);
    }

    process::exit(result.code);
}
