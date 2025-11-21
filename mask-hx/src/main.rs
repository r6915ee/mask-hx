//! A program made to handle [Haxe](https://haxe.org) versions.
//!
//! `mask-hx` aims to simplify [Haxe](https://haxe.org) version
//! management. [Haxe](https://haxe.org), unlike most other toolchains,
//! does not play well with projects as a result of its
//! versioning mechanism including syntax changes. `mask-hx`
//! aims to simplify the process of version management with
//! [Haxe](https://haxe.org).

use std::{env, io::Error, process};

use clap::{Arg, ArgAction, ArgMatches, Command, arg, command};

use libmask::*;

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
                .arg(arg!(<HAXE_VERSION> "The Haxe version to switch to"))
                .arg(
                    Arg::new("skip-check")
                        .short('u')
                        .long("skip-check")
                        .help("Skips checking the existence of a Haxe installation")
                        .action(ArgAction::SetTrue),
                ),
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
                    arg!([ARGUMENTS]... "Specify the arguments to pass to the compiler")
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
                    arg!([ARGUMENTS]... "Specify the arguments to pass to Haxelib")
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
    let mut message: Box<String> = Box::new(
        "invalid subcommand or no subcommand was passed; try running mask-hx help".to_string(),
    );
    let mut config_path: Option<&str> = None;
    let mut exit_code: i32 = 0;
    let mut force_exit_log: bool = false;

    let config: Option<Config> = if let Some(version) = matches.get_one::<String>("explicit") {
        Some(Config(HaxeVersion(version.clone())))
    } else if let Ok(data) = env::var("MASK_VERSION") {
        Some(Config(HaxeVersion(data)))
    } else if let Some(version) = matches.get_one::<String>("config") {
        config_path = Some(version.as_str());
        Some(Config::new(Some(version)).unwrap_or_default())
    } else {
        Config::new(None).ok()
    };

    /// Shorthand method for executing a program.
    fn execute(params: &ArgMatches, config: Config, prog: &str) -> Result<(String, i32), Error> {
        let mut args: Vec<String> = Vec::new();
        if let Some(list) = params.get_many::<String>("ARGUMENTS") {
            for i in list {
                args.push(i.to_string());
            }
        }

        match haxe_exec(args, config, Some(prog.to_string())) {
            Ok(output) => Ok((
                if output.status.code().is_none() {
                    format!("Successfully started {}, but program was interrupted", prog)
                } else {
                    "".to_string()
                },
                output.status.code().unwrap_or(143),
            )),
            Err(e) => Err(e),
        }
    }

    /// Checks the validity of a configuration, and exits if it is invalid.
    fn check_config_validity(config: &Option<Config>) {
        if let Some(data) = config {
            if data.0.0.is_empty() {
                println!("mask-hx: No Haxe version specified");
            } else {
                return;
            }
        } else {
            println!(
                "mask-hx: Impossible to construct valid configuration; \
                for starters, use the --explicit flag to specify the version"
            );
        }
        process::exit(2);
    }

    if matches.subcommand_matches("check").is_some() {
        check_config_validity(&config);
        match config.as_ref().unwrap().0.get_path_installed() {
            Ok(_) => {
                *message = format!("Haxe version {} is ready to use", config.unwrap().0.0);
                force_exit_log = true;
            }
            Err(e) => {
                *message = e.to_string();
                exit_code = 2;
            }
        }
    } else if let Some(data) = matches.subcommand_matches("switch") {
        let store: Result<(), Error> = if data.get_flag("skip-check") {
            Config::write(config_path, data.get_one::<String>("HAXE_VERSION").unwrap())
        } else {
            Config::safe_write(config_path, data.get_one::<String>("HAXE_VERSION").unwrap())
        };
        match store {
            Ok(_) => {
                *message = format!(
                    "successfully switched config {} to use Haxe version {}",
                    config_path.unwrap_or("./.mask"),
                    data.get_one::<String>("HAXE_VERSION").unwrap()
                );
                force_exit_log = true;
            }
            Err(e) => {
                *message = e.to_string();
                exit_code = 1;
            }
        }
    } else if let Some(params) = matches.subcommand_matches("exec") {
        check_config_validity(&config);
        let results: (String, i32) = match execute(params, config.unwrap(), "haxe") {
            Ok(data) => data,
            Err(e) => (format!("Execution error: {}", e), 1),
        };
        *message = results.0;
        exit_code = results.1;
    } else if let Some(params) = matches.subcommand_matches("lib") {
        check_config_validity(&config);
        let results: (String, i32) = match execute(params, config.unwrap(), "haxelib") {
            Ok(data) => data,
            Err(e) => (format!("Execution error: {}", e), 1),
        };
        *message = results.0;
        exit_code = results.1;
    } else {
        force_exit_log = true;
    };

    if exit_code != 0 || force_exit_log {
        println!("mask-hx: {}", *message);
    }

    process::exit(exit_code);
}
