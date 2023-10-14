pub struct Camera {
    _background_colour: [f32; 4],
    _position: [f32; 2],
    _zoom: f32,
}

impl Camera {
    pub fn new(background_colour: [f32; 4], position: [f32; 2], zoom: f32) -> Self {
        Self {
            _background_colour: background_colour,
            _position: position,
            _zoom: zoom,
        }
    }
}
