use std::io;
use std::process::ExitStatus;
use std::string::FromUtf8Error;

use lazy_static::lazy_static;
use quick_error::quick_error;
use regex::{Match, Regex};

lazy_static! {
    static ref AUX_FILE: Regex = {
        // Lines like:
        //      *(/tmp/texput.aux)
        Regex::new(r"^\*\(([^)]+\.aux)\)$").unwrap()
    };

    static ref LOG_FILE: Regex = {
        // Lines like:
        //      Transcript written on /tmp/texput.log.
        Regex::new(r"^Transcript written on (.+\.log).$").unwrap()
    };
}

pub fn first_group<'a>(re: &Regex, line: &'a str) -> Option<&'a str> {
    re.captures(line).and_then(|c| c.get(1)).map(|m| m.as_str())
}

pub fn aux_file(line: &str) -> Option<&str> {
    first_group(&AUX_FILE, line)
}

pub fn log_file(line: &str) -> Option<&str> {
    first_group(&LOG_FILE, line)
}

quick_error! {
    #[derive(Debug)]
    pub enum RunError {
        FailedToStart(engine: String) {
            display("Failed to start TeX engine {:?}.", engine)
        }

        TexFailed(status: ExitStatus) {
            display("TeX engine didn't complete successfully; exit status {:?}.", status)
        }

        TexStderr(err: String) {
            display("TeX wrote to stderr: {}", err)
        }

        NoPipe {
            display("Didn't capture stdin/stdout/stderr pipe.")
        }

        FromUtf8(err: FromUtf8Error) {}

        Io(err: io::Error) {}
    }
}

#[derive(Default)]
pub struct LatexJob {
    /// Determines if the current line should be displayed?
    displaying: bool,
    aux_file: Option<String>,
    log_file: Option<String>,
}

impl LatexJob {
    pub fn should_display(&mut self, line: &str) -> bool {
        if line.starts_with('>') || line.starts_with("*>") {
            self.displaying = true;
        } else if line.starts_with('?') || line.starts_with("<recently read>") {
            self.displaying = false;
        }
        self.displaying
    }

    pub fn process(&mut self, line: &str) -> bool {
        if self.should_display(line) {
            true
        } else {
            if self.aux_file.is_none() {
                self.aux_file = aux_file(line).map(Into::into);
            }

            if self.log_file.is_none() {
                self.log_file = log_file(line).map(Into::into);
            }

            false
        }
    }
}

struct TexCommand {
    // The name, including the backslash.
    name: String,
    // The parameters, i.e. #1#2#3
    parameters: String,
    // The definition, or expansion text.
    definition: String,
}
