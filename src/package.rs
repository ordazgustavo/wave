use std::fmt;

pub struct Package {
    pub name: String,
    pub version: Option<String>,
    pub main: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n  \"name\": \"{}\"\n}}", self.name)
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Some(String::from("1.0.0")),
            main: Some(String::from("index.js")),
            author: Some(String::from("")),
            license: Some(String::from("MIT")),
        }
    }
}
