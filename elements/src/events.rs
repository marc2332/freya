use glutin::keyboard::Key;

pub type KeyCode = Key<'static>;

#[derive(Debug)]
pub struct KeyboardData {
    pub code: KeyCode,
}

impl KeyboardData {
    pub fn new(code: KeyCode) -> Self {
        Self { code }
    }
}

impl KeyboardData {
    pub fn to_text(&self) -> Option<&str> {
        self.code.to_text()
    }
}
