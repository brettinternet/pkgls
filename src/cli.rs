use crate::{
    config::OutputFormat,
    error::*,
    logger::{filter_level_occurences, filter_level_value},
};
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
            // .global_setting(AppSettings::DeriveDisplayOrder)
            // .global_setting(AppSettings::UnifiedHelpMessage)
            // .global_setting(AppSettings::HidePossibleValuesInHelp)
            // .setting(AppSettings::ArgsNegateSubcommands)
            // .setting(AppSettings::AllowExternalSubcommands)
            // .setting(AppSettings::DisableHelpSubcommand)
            // .setting(AppSettings::VersionlessSubcommands)
            .max_term_width(100)
            .author("Brett (https://github.com/brettinternet/pkgls)")
            .about("List installed packages")
            .arg(
                Arg::new("output")
                    .about("Sets an optional output file")
                    .index(1),
            )
            .arg(
                Arg::new("program")
                    .short('p')
                    .long("program")
                    .about("Set which package manager to use"),
            )
            .arg(
                Arg::new("log")
                    .short('l')
                    .long("log")
                    .default_value("off")
                    .multiple_occurrences(true)
                    .about("Set a log level"),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .about("Silence stdout"),
            )
            .subcommand(
                App::new("print")
                    .about("Print package list to stdout")
                    .arg(Arg::new("list").short('l').about("lists test values")),
            );

        let matches = app.get_matches();

        #[cfg(debug_assertions)]
        debug!("Matches: {:?}", matches);

        Self { matches, color }
    }

    /// Quiet
    ///
    /// Silence all output
    ///
    /// bool whether output should be silenced
    pub fn get_quiet(&self) -> bool {
        self.matches.value_of("quiet").is_some()
    }

    /// Log level
    ///
    /// Supported log levels are off, debug, info, warn and error
    ///
    /// Default log level is "off"
    pub fn get_log_level(&self) -> LevelFilter {
        if self.get_quiet() {
            LevelFilter::Off
        } else if let Some(level) = self.matches.value_of("log") {
            filter_level_value(level)
        } else {
            let count = self.matches.occurrences_of("log");
            filter_level_occurences(count)
        }
    }

    /// Output format
    ///
    /// Defaults to Stdout
    pub fn get_output_format<'a>(&'a self) -> Result<OutputFormat> {
        // You can check the value provided by positional arguments, or option arguments
        match self.matches.value_of("output") {
            Some(o) if o.to_lowercase() == OutputFormat::Stdout.to_string().to_lowercase() => {
                Ok(OutputFormat::Stdout)
            }
            None => {
                debug!("using default output of '{}'", OutputFormat::Stdout);
                Ok(OutputFormat::Stdout)
            }
            Some(o) => Err(ErrorKind::UnsupportedOutputFormat(o.to_string()).into()),
        }
    }

    // pub fn get_other<'a>(&'a mut self) {
    //     if let Some(c) = self.matches.value_of("config") {
    //         println!("Value for config: {}", c);
    //     }

    //     // You can check for the existence of subcommands, and if found use their
    //     // matches just as you would the top level app
    //     if let Some(ref matches) = self.matches.subcommand_matches("test") {
    //         // "$ myapp test" was run
    //         if matches.is_present("list") {
    //             // "$ myapp test -l" was run
    //             println!("Printing testing lists...");
    //         } else {
    //             println!("Not printing testing lists...");
    //         }
    //     }

    //     // Continued program logic goes here...
    // }
}
