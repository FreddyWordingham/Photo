pub struct State {
    pub save_image: bool,
}

impl State {
    pub fn new() -> Self {
        Self { save_image: false }
    }
}
