pub struct Canvas {
    width: f64,
    height: f64,
    aspect_ratio: f64,
    tiles: Vec<(f64, f64, f64, f64)>,
    content: Vec<String>
}

impl Canvas {
    pub fn new(width: f64, height:f64) -> Canvas {
        Canvas {
            width,
            height,
            aspect_ratio: width/height,
            tiles: Vec::new(),
            content: Vec::new()
        }
    }

    pub fn dimensions(&self) -> (&f64, &f64) {
        (&self.width, &self.height)
    }

    pub fn aspect_ratio(&self) -> &f64 {
        &self.aspect_ratio
    }
}