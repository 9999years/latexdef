use ansi_term::{
    Color::{Black, Blue, Purple, Yellow},
    Style as TermStyle,
};
use clap::{clap_app, App};

use latexdef::document::DocumentConfig;
use latexdef::run::{LatexJob, RunError};

fn is_file(s: String) -> Result<(), String> {
    if s.ends_with(".tex") || s.ends_with(".sty") || s.ends_with(".cls") {
        Ok(())
    } else {
        Err("".into())
    }
}

fn clap<'a, 'b>() -> App<'a, 'b> {
    clap_app!(latexdef =>
        (version: "0.1.0")
        (author: "Rebecca Turner <rbt@sent.as>")
        (about: "Prints definitions of LaTeX macros.")
        (@arg VERBOSE: --verbose "Print verbose output")
        (@arg ENGINE: --engine <ENGINE> default_value[latex] "TeX engine to run")
        (@arg DOCUMENTCLASS: --documentclass <CLASS> default_value[article] "Document class to use")
        (@arg EXPL3: -e --expl3 "Enable LaTeX3e features with the expl3 package")
        (@arg MATH: --math "Load common math packages (amsmath, amssymb, amsthm, mathtools)")
        (@arg PACKAGES: -p --packages [PACKAGE] ... +takes_value "Packages to load")
        (@arg COMMANDS: <COMMAND> ... "Commands to show definitions of")
    )
}

fn main() -> Result<(), RunError> {
    let matches = clap().get_matches();
    let verbose = matches.is_present("VERBOSE");
    let output = DocumentConfig::from(matches).run()?;
    for cmd in LatexJob::new(&output).verbose(verbose) {
        if cmd.is_primitive() {
            println!("{} is primitive.", Purple.bold().paint(&cmd.name),);
        } else if !cmd.is_undefined() {
            if cmd.definition.is_empty() {
                println!(
                    "{} {} {}",
                    Blue.bold().paint(&cmd.name),
                    Black.bold().paint("="),
                    TermStyle::new().bold().paint(&cmd.kind),
                );
            } else {
                print!(
                    "{} {} {} ",
                    Blue.bold().paint(&cmd.name),
                    Black.bold().paint(format!("(type: {})", &cmd.kind)),
                    Black.bold().paint("="),
                );
                if cmd.has_parameters() {
                    print!(
                        "{}{}",
                        TermStyle::new().bold().paint(&cmd.parameters),
                        Black.bold().paint(" -> ")
                    );
                }
                println!("{}", cmd.definition);
            }
        } else {
            let name = Yellow.paint(&cmd.name);
            println!("{} {}", name, Black.bold().paint("is undefined."));
        }
    }
    Ok(())
}
