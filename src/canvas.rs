use agent::canvas::canvasagent::CanvasAgent;
use tile::Tile;

pub struct Canvas<'a> {
    width: f64,
    height: f64,
    aspect_ratio: f64,
    center_of_mass: (f64, f64),
    tiles: Vec<Tile<'a>>,
    agents: Vec<Box<CanvasAgent>>,
}

impl<'a> Canvas<'a> {
    pub fn new(width: f64, height:f64) -> Canvas<'a> {
        Canvas {
            width,
            height,
            aspect_ratio: width/height,
            center_of_mass: (width/2.0, height/2.0),
            tiles: Vec::new(),
            agents: Vec::new()
        }
    }

    pub fn add_tile(&mut self, from: (f64, f64), to: (f64, f64)){
        let tile = Tile::new(from, to);
        self.tiles.push(tile);
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

    pub fn tiles_borrow(&self) -> &Vec<Tile<'a>> {
        &self.tiles
    }

    pub fn register_agent(&mut self, agent: Box<CanvasAgent>) {
        self.agents.push(agent);
    }
}