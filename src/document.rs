use std::env;
use std::ffi::OsString;

use std::io::Write;
use std::process::{Child, Command, Stdio};

use clap::ArgMatches;

use crate::package::Package;
use crate::run::RunError;

pub struct DocumentConfig {
    pub engine: String,
    pub documentclass: Option<String>,
    pub packages: Vec<Package>,
    pub commands: Vec<String>,
    pub expl3: bool,
}

impl Default for DocumentConfig {
    fn default() -> Self {
        DocumentConfig {
            engine: String::from("latex"),
            documentclass: Some(String::from("article")),
            packages: Vec::new(),
            commands: Vec::new(),
            expl3: false,
        }
    }
}

impl DocumentConfig {
    pub fn render(&self) -> String {
        let mut ret = String::new();
        if let Some(dc) = &self.documentclass {
            ret.push_str(&format!("\\documentclass{{{}}}\n", dc));
        }
        ret.push_str("\\makeatletter\n");
        for package in &self.packages {
            if let Some(opts) = &package.options {
                ret.push_str(&format!("\\usepackage[{}]{{{}}}\n", opts, package.name));
            } else {
                ret.push_str(&format!("\\usepackage{{{}}}\n", package.name));
            }
        }
        if self.expl3 {
            ret.push_str("\\usepackage{expl3}\n\\ExplSyntaxOn\n")
        }
        for command in &self.commands {
            ret.push_str(&format!(
                "\\expandafter\\show\\csname {}\\endcsname\n\n",
                command
            ));
        }
        ret.push_str("\\makeatother\n");
        if self.expl3 {
            ret.push_str("\\ExplSyntaxOff\n")
        }
        ret.push_str("\\begin{document}\n\\end{document}\n\n");
        ret
    }

    fn command(&self) -> Result<Child, RunError> {
        let mut out_dir = OsString::from("-output-directory=");
        out_dir.push(env::temp_dir().into_os_string());
        let mut cmd = Command::new(&self.engine)
            .arg(out_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(RunError::Io)?;

        cmd.stdin
            .as_mut()
            .ok_or(RunError::NoPipe)?
            .write_all(&self.render().into_bytes())
            .map_err(RunError::Io)?;

        Ok(cmd)
    }

    pub fn run(&self) -> Result<String, RunError> {
        let cmd = self.command()?;
        let output = cmd.wait_with_output().map_err(RunError::Io)?;
        if !output.stderr.is_empty() {
            return Err(RunError::TexStderr(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        // if !output.status.success() {
        // return Err(RunError::TexFailed(output.status));
        // }
        String::from_utf8(output.stdout).map_err(RunError::FromUtf8)
    }
}

impl<'a> From<ArgMatches<'a>> for DocumentConfig {
    fn from(matches: ArgMatches<'a>) -> Self {
        let mut doc = DocumentConfig {
            engine: matches
                .value_of("ENGINE")
                .expect("ENGINE should have a default value.")
                .into(),
            documentclass: Some(
                matches
                    .value_of("DOCUMENTCLASS")
                    .expect("DOCUMENTCLASS should have a default value.")
                    .into(),
            ),
            expl3: matches.is_present("EXPL3"),
            ..Default::default()
        };

        if matches.is_present("MATH") {
            // Add common mathematical packages.
            doc.packages.append(&mut vec![
                "amsmath".into(),
                "amssymb".into(),
                "amsthm".into(),
                "mathtools".into(),
            ]);
        }

        if let Some(packages) = matches.values_of("PACKAGES") {
            for package in packages {
                doc.packages.push(package.into());
            }
        }

        if let Some(commands) = matches.values_of("COMMANDS") {
            for command in commands {
                doc.commands.push(command.into());
            }
        }

        doc
    }
}
