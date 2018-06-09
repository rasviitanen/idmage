use std::fmt;

pub struct Canvas {
    width: f64,
    height: f64,
    aspect_ratio: f64,
    tiles: Vec<Tile>,
}

pub struct Tile {
    from: (f64, f64),
    to: (f64, f64)
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile<({},{}), ({},{})>", self.from.0, self.from.1, self.to.0, self.to.1)
    }
}

impl Canvas {
    pub fn new(width: f64, height:f64) -> Canvas {
        Canvas {
            width,
            height,
            aspect_ratio: width/height,
            tiles: Vec::new()
        }
    }

    pub fn add_tile(&mut self, from: (f64, f64), to: (f64, f64)) {
        self.tiles.push(Tile { from, to });
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Canvas<dimensions: ({},{})>", self.width, self.height)
    }
}
