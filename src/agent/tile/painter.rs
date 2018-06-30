use agent::tile::tileagent::TileAgent;
use tile::Tile;
use agent::tile::request::Request;
use graphic::Graphic;

pub struct Painter {
    request: Option<Request>,
}

impl Painter {
    pub fn new() -> Painter {
        Painter {
            request: None,
        }
    }
}

impl TileAgent for Painter {
    fn update(&mut self, tile: &Tile) {
        self.request = Some(request!(move |tile| {
            let mut circle = Graphic::new("circle");
            circle.add_attr("cx=\"500\"");
            circle.add_attr("cy=\"500\"");
            circle.add_attr("r=\"120\"");
            
            tile.add_graphic(circle);
        }));
    }

    fn execute(&mut self, tile: &mut Tile) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Balancer");
                req.execute(tile);
            },
            _ => {
                println!("{:?}", "No request to be executed by Balancer");
            }
        }
        self.request = None;
    }
}