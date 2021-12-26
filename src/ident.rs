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
