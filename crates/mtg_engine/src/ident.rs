use std::borrow::Borrow;
use std::fmt::{self, Debug, Display};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
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

impl Borrow<str> for Ident {
    fn borrow(&self) -> &str {
        self.0.borrow()
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
