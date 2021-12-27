use std::fmt::{self, Debug, Display};

use smol_str::SmolStr;

pub struct Ident(SmolStr);

impl Ident {
    pub fn new(source: &str) -> Self {
        Ident(SmolStr::new(source))
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
