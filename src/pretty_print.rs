use std::fmt;

struct PrettyPrinterSettings {
    /// Number of spaces to indent blocks.
    pub indent: usize,
}

impl Default for PrettyPrinterSettings {
    fn default() -> Self {
        Self { indent: 2 }
    }
}

#[derive(PartialEq)]
enum Brace {
    Open,
    Close,
}

impl Brace {
    pub fn new_indent(&self, indent_level: usize) -> usize {
        match self {
            Brace::Open => indent_level + 1,
            Brace::Close => indent_level - 1,
        }
    }
}

impl From<&Brace> for char {
    fn from(b: &Brace) -> Self {
        match b {
            Brace::Open => '{',
            Brace::Close => '}',
        }
    }
}

pub struct PrettyPrinter<'a> {
    settings: PrettyPrinterSettings,
    src: &'a str,
    brace_indices: Vec<(usize, Brace)>,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(src: &'a str) -> Option<Self> {
        braces_are_balanced(src).map(|brace_indices| Self {
            settings: PrettyPrinterSettings::default(),
            src,
            brace_indices,
        })
    }

    fn indent(&self, level: usize) -> String {
        " ".repeat(self.settings.indent * level)
    }
}

impl fmt::Display for PrettyPrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The current indentation level.
        let mut level: usize = 0;
        // The previous offset in the string to format.
        let mut prev_ofs = 0;
        for (ofs, brace) in &self.brace_indices {
            write!(
                f,
                "{}{}",
                self.indent(level),
                self.src
                    .get(prev_ofs..*ofs)
                    .expect("braces_are_balanced should never return invalid indices."),
            )?;
            level = brace.new_indent(level);
            match brace {
                Brace::Open => {
                    // Write at EOL
                    writeln!(f, "{}", char::from(brace))?;
                }
                Brace::Close => {
                    // Write on next line
                    writeln!(f, "\n{}{}", self.indent(level), char::from(brace))?;
                }
            }
            prev_ofs = *ofs + 1;
        }
        Ok(())
    }
}

/// Checks if {} are balanced in a string.
fn braces_are_balanced(text: &str) -> Option<Vec<(usize, Brace)>> {
    let mut indices = Vec::new();
    for (i, c) in text.char_indices() {
        match c {
            '{' => {
                indices.push((i, Brace::Open));
            }
            '}' => {
                indices.push((i, Brace::Close));
            }
            _ => {}
        }
    }
    let count_for_type = |kind| indices.iter().filter(|(_, b)| *b == kind).count();
    if count_for_type(Brace::Open) == count_for_type(Brace::Close) {
        Some(indices)
    } else {
        None
    }
}
