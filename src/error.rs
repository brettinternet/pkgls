use error_chain::error_chain;
use std::io::Write;

error_chain! {
    foreign_links {
        Clap(::clap::Error);
        Io(::std::io::Error);
        ParseIntError(::std::num::ParseIntError);
        Utf8Error(::std::str::Utf8Error);
    }

    errors {
        UndetectedManager(target: String) {
            description("unable to detect package manager"),
            display("Unable to detect package manager for {}", target)
        }
        UnsupportedManager(input: String) {
            description("unsupported package manager"),
            display("Unsupported package manager '{}'", input)
        }
        PackagesNotFound(manager: String) {
            description("packages not found")
            display("Packages were not found for {}", manager)
        }
        UnsupportedOutputFormat(input: String) {
            description("unsupported output format"),
            display("Unsupported output format '{}'", input)
        }
        InterruptionStdErr(error: String) {
            description("interruption from stderr"),
            display("Interruption from stderr: {}", error),
        }
    }
}

pub fn default_error_handler(error: &Error, output: &mut dyn Write) {
    match error {
        Error(ErrorKind::Io(ref io_error), _)
            if io_error.kind() == ::std::io::ErrorKind::BrokenPipe =>
        {
            ::std::process::exit(0);
        }
        _ => {
            writeln!(output, "{}", error).ok();
        }
    };
}
