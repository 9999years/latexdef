#[derive(Debug, PartialEq)]
pub struct Package {
    pub name: String,
    pub options: Option<String>,
}

impl<S: Into<String>> From<S> for Package {
    fn from(s: S) -> Self {
        Package {
            name: s.into(),
            options: None,
        }
    }
}
