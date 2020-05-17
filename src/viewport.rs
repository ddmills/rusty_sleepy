pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            x: 2.0,
            y: 2.0,
            width: 15.0,
            height: 6.0,
        }
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x > self.x && x < self.right() && y > self.y && y < self.bottom()
    }
}
