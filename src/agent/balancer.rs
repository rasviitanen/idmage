use agent::agent::Agent;
use canvas::Canvas;

pub struct Balancer {
    request: Option<Box<Fn()>>,
    balance_constant: usize,
}

impl Balancer {
    pub fn new() -> Balancer {
        Balancer {
            request: None,
            balance_constant: 10,
        }
    }
}

impl Agent for Balancer {
    fn update(&mut self, canvas: &mut Canvas) {
        // Calculate center of mass
        let mut cx: f64 = 0.0;
        let mut cy: f64 = 0.0;
        let mut total_mass = 0.0;
        for tile in canvas.tiles() {
            let (tile_cx, tile_cy) = tile.center();
            cx += tile_cx*tile.weight();
            cy += tile_cy*tile.weight();
            total_mass += tile.weight();
        }
        self.request = Some(Box::new(|| println!("{:?}", "Dynamic closure TEST in balancer")));
    }

    fn execute(&self) {
        match &self.request {
            Some(req) => {
                req();
            },
            _ => {}
        }
    }
}