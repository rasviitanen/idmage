use agent::tile::tileagent::TileAgent;
use graphic::Graphic;

pub struct Tile<'a> {
    from: (f64, f64),
    to: (f64, f64),
    weight: f64,
    graphics: Vec<Graphic<'a>>,
    agents: Vec<Box<TileAgent>>,
}

impl<'a> Tile<'a> {
    pub fn new(from: (f64, f64), to: (f64, f64)) -> Tile<'a> {
        Tile {
            from,
            to,
            weight: 0.0,
            graphics: Vec::new(),
            agents: Vec::new()
        }
    }
    
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

    pub fn register_agent(&mut self, agent: Box<TileAgent>) {
        self.agents.push(agent);
    }
}