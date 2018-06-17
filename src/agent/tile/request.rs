use canvas::Tile;

macro_rules! request {
    ($e:expr) => {{
        Request { code: Box::new($e) }
    }};
}

pub struct Request {
    pub code: Box<Fn(&mut Tile)>,
}

impl Request {
    pub fn execute(&self, tile: &mut Tile) {
        (self.code)(tile);
    }
}