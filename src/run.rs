use std::io;
use std::process::ExitStatus;
use std::str::Lines;
use std::string::FromUtf8Error;

use ansi_term::Color::Black;
use lazy_static::lazy_static;
use quick_error::quick_error;
use regex::Regex;

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

    static ref MACRO_NAME: Regex = {
        Regex::new(r"^\*> (\\.|\\[a-zA-Z:_@]+)=(.*)$").unwrap()
    };

    static ref MACRO_PARAMS: Regex = {
        Regex::new(r"^((?:#\d)*)->").unwrap()
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

#[derive(PartialEq, Debug)]
enum LineKind {
    Ignore,
    MacroName,
    MacroStart,
    MacroContinue,
}

impl Default for LineKind {
    fn default() -> Self {
        LineKind::Ignore
    }
}

#[derive(Default, Debug, Clone)]
pub struct TexCommand {
    // The name, including the backslash.
    pub name: String,
    pub kind: String,
    // The parameters, i.e. #1#2#3
    pub parameters: String,
    // The definition, or expansion text.
    pub definition: String,
}

impl TexCommand {
    pub fn is_undefined(&self) -> bool {
        self.definition.is_empty() && (self.kind == "\\relax." && self.name != "\\relax")
    }

    pub fn is_primitive(&self) -> bool {
        self.name == self.kind.trim_end_matches('.')
    }

    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }
}

#[derive(Debug)]
pub struct LatexJob<'a> {
    lines: Lines<'a>,
    line: LineKind,
    current: TexCommand,
    verbose: bool,
}

impl<'a> LatexJob<'a> {
    pub fn new(s: &'a str) -> Self {
        LatexJob {
            lines: s.lines(),
            line: LineKind::Ignore,
            current: TexCommand::default(),
            verbose: false,
        }
    }

    pub fn verbose(&mut self, is_verbose: bool) -> &mut Self {
        self.verbose = is_verbose;
        self
    }
}

impl<'a> Iterator for LatexJob<'a> {
    type Item = TexCommand;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        if self.verbose {
            println!("{}", Black.bold().paint(line));
        }
        if line.starts_with("<recently read>") {
            // Finished with this definition, return.
            self.line = LineKind::Ignore;
            let last_char = self.current.definition.pop();
            match last_char {
                Some('.') => {}
                Some(c) => {
                    panic!(
                        "Expected to find '.' as last char of definition, instead found '{:?}'.\n{:?}",
                        c, self.current
                    );
                }
                None => {}
            }
            let ret = self.current.clone();
            self.current = TexCommand::default();
            Some(ret)
        } else if let Some(caps) = MACRO_NAME.captures(line) {
            // Starting a new definition.
            self.line = LineKind::MacroName;
            self.current = TexCommand {
                name: caps.get(1).unwrap().as_str().into(),
                kind: caps.get(2).unwrap().as_str().trim_end_matches(':').into(),
                ..Default::default()
            };
            self.next()
        } else if let Some(i) = line.find("->") {
            // Parameters; first line of a new definition.
            self.line = LineKind::MacroStart;
            let (params, rest) = line.split_at(i);
            self.current.parameters = params.into();
            self.current.definition.push_str(rest.get(2..).unwrap());
            self.next()
        } else {
            match self.line {
                LineKind::Ignore => {}
                LineKind::MacroName => {
                    panic!(
                        "Impossible state; should have found macro parameters! {:?}\nLine: {:?}",
                        self.current, line
                    );
                }
                LineKind::MacroStart => {
                    // Just past the parameters line.
                    self.line = LineKind::MacroContinue;
                    self.current.definition.push_str(line);
                }
                LineKind::MacroContinue => {
                    self.current.definition.push_str(line);
                }
            }
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_name_test() {
        assert!(MACRO_NAME.is_match("*> \\@author=\\macro:"));
        assert!(MACRO_NAME.is_match("*> \\parskip=\\parskip."));

        let caps = MACRO_NAME.captures("*> \\bool_new:N=\\macro:").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "\\bool_new:N");
        assert_eq!(caps.get(2).unwrap().as_str(), "\\macro:");
    }

    #[test]
    fn macro_params_test() {
        assert!(MACRO_PARAMS.is_match("#1->xyz..."));
        assert!(MACRO_PARAMS.is_match("#1#2#3->xyz..."));
        assert!(MACRO_PARAMS.is_match("-> zoooop"));
        assert!(!MACRO_PARAMS.is_match("#10-> zoooop"));
    }
}
