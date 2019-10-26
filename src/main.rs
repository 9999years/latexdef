use ansi_term::{
    Color::{Black, Blue, Purple, White, Yellow},
    Style as TermStyle,
};
use clap::{clap_app, App, ArgMatches};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

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

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("tex").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    let output = DocumentConfig::from(matches).run()?;
    let job = LatexJob::new(&output);
    for cmd in job {
        if cmd.is_primitive() {
            println!("{} is primitive.", Purple.bold().paint(&cmd.name),);
        } else if cmd.is_defined() {
            print!(
                "{} {} ",
                Blue.bold().paint(&cmd.name),
                Black.bold().paint("=")
            );
            if cmd.has_parameters() {
                print!(
                    "{}{}",
                    TermStyle::new().bold().paint(&cmd.parameters),
                    Black.bold().paint(" -> ")
                );
            }
            println!("{}", cmd.definition);
        } else {
            let name = Yellow.paint(&cmd.name);
            println!("{} {}", name, Black.bold().paint("is undefined"));
        }
    }
    Ok(())
}
