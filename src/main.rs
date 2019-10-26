use clap::{clap_app, App, ArgMatches};

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
        (version: "0.0.1")
        (author: "Rebecca Turner <rbt@sent.as>")
        (about: "Prints definitions of LaTeX macros.")
        (@arg ENGINE: --engine <ENGINE> default_value[latex] "TeX engine to run.")
        (@arg DOCUMENTCLASS: --documentclass <CLASS> default_value[article] "Document class to use")
        (@arg EXPL3: -e --expl3 "Enable LaTeX3e features with the expl3 package")
        (@arg MATH: --math "Load common math packages (amsmath, amssymb, amsthm, mathtools)")
        (@arg PACKAGES: -p --packages [PACKAGE] ... +takes_value "Packages to load")
        (@arg COMMANDS: <COMMAND> ... "Commands to show definitions of")
    )
}

fn main() -> Result<(), RunError> {
    let matches = clap().get_matches();
    let output = DocumentConfig::from(matches).run()?;
    let mut job = LatexJob::default();
    for line in output.lines() {
        if job.should_display(line) {
            println!("{}", line);
        }
    }
    Ok(())
}
