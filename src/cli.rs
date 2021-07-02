use crate::app::Procedure;
use crate::logger::filter_level_occurences;
use clap::{App, AppSettings, Arg, ArgMatches};
use log::LevelFilter;
use std::env;

/// Cli input parsed by Clap and associated input interpreters
#[derive(Debug)]
pub struct Cli {
    /// Clap argument matches
    matches: ArgMatches,

    /// If color is supported
    pub color: bool,
}

impl Cli {
    /// TODO: support -u uninstall, -i install, -e explicitly installed filter, -v use versions
    pub fn new() -> Self {
        let color = env::var_os("NO_COLOR").is_none();
        let cli_color_setting = if color {
            AppSettings::ColoredHelp
        } else {
            AppSettings::ColorNever
        };

        let app = App::new("pkgls")
            .version("0.1.0")
            // .license("MIT") // unreleased
            .global_setting(cli_color_setting)
            .global_setting(AppSettings::UnifiedHelpMessage)
            .setting(AppSettings::ArgsNegateSubcommands)
            .setting(AppSettings::DisableHelpSubcommand)
            .setting(AppSettings::VersionlessSubcommands)
            .max_term_width(100)
            .author("Brett (https://github.com/brettinternet/pkgls)")
            .about("List installed packages")
            .arg(
                Arg::new("program")
                    .short('p')
                    .long("program")
                    .takes_value(true)
                    .possible_values(&["pacman"])
                    .about("Explicitly set which package manager to use"),
            )
            .arg(
                Arg::new("log")
                    .short('l')
                    .long("log")
                    .takes_value(false)
                    .multiple_occurrences(true)
                    .conflicts_with("quiet")
                    .about("Increment a log level"),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .conflicts_with("log")
                    .about("Silence stdout and stderr"),
            )
            .subcommand(
                App::new("list")
                    .about("List installed packages or save to file")
                    .arg(
                        Arg::new("OUTPUT")
                            .index(1)
                            .about("Filename to write package names"),
                    )
                    .arg(
                        Arg::new("force")
                            .short('f')
                            .long("force")
                            .requires("OUTPUT")
                            .about("Force overwrite the output if it already exists"),
                    ),
            );

        let matches = app.get_matches();
        Self { matches, color }
    }

    /// Subcommand procedure
    ///
    /// Derive the procedure from the subcommands
    pub fn get_procedure(&self) -> Procedure {
        if self.matches.is_present("add") {
            Procedure::List
        } else {
            debug!("Running 'list' subcommand by default");
            Procedure::List
        }
    }

    /// Output filename
    pub fn get_output(&self) -> Option<&str> {
        if let Some(list_matches) = self.matches.subcommand_matches("list") {
            list_matches.value_of("output")
        } else {
            None
        }
    }

    /// Force
    ///
    /// Force overwriting target file
    ///
    /// bool whether force is requested
    pub fn get_force(&self) -> bool {
        if let Some(list_matches) = self.matches.subcommand_matches("list") {
            list_matches.is_present("force")
        } else {
            false
        }
    }

    /// Quiet
    ///
    /// Silence all output
    ///
    /// bool whether output should be silenced
    pub fn get_quiet(&self) -> bool {
        self.matches.is_present("quiet")
    }

    /// Log level
    ///
    /// Supported log levels are off, debug, info, warn and error
    ///
    /// Default log level is "off"
    pub fn get_log_level(&self) -> LevelFilter {
        if self.get_quiet() {
            LevelFilter::Off
        } else {
            let count = self.matches.occurrences_of("log");
            filter_level_occurences(count)
        }
    }

    /// Package manager program
    pub fn get_program(&self) -> Option<&str> {
        self.matches.value_of("program")
    }
}
