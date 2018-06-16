use graphic::Graphic;

#[derive(Debug)]
pub struct Tile<'a> {
    from: (f64, f64),
    to: (f64, f64),
    weight: f64,
    graphics: Vec<Graphic<'a>>,
}

impl<'a> Tile<'a> {
    pub fn graphics(&self) -> &Vec<Graphic<'a>> {
        &self.graphics
    }

    pub fn add_graphic(&mut self, graphic: Graphic<'a>) {
        self.graphics.push(graphic);
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn center(&self) -> (f64, f64) {
        let cx = self.from.0 + (self.to.0 - self.from.0)/2.0;
        let cy = self.from.1 + (self.to.1 - self.from.1)/2.0;
        (cx, cy)
    }
}

#[derive(Debug)]
pub struct Canvas<'a> {
    width: f64,
    height: f64,
    aspect_ratio: f64,
    center_of_mass: (f64, f64),
    tiles: Vec<Tile<'a>>,
}

impl<'a> Canvas<'a> {
    pub fn new(width: f64, height:f64) -> Canvas<'a> {
        Canvas {
            width,
            height,
            aspect_ratio: width/height,
            center_of_mass: (width/2.0, height/2.0),
            tiles: Vec::new()
        }
    }

    pub fn add_tile(&mut self, from: (f64, f64), to: (f64, f64)) {
        self.tiles.push(Tile { from, to, weight: 0.0, graphics: Vec::new() });
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn set_center_of_mass(&mut self, cx: f64, cy: f64) {
        self.center_of_mass = (cx, cy);
    }

    pub fn center_of_mass(&self) -> (f64, f64) {
        self.center_of_mass
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}