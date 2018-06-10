use agent::agent::Agent;
use canvas::Canvas;

pub struct Balancer {
    request: Option<Box<Fn(&mut Canvas)>>,
}

impl Balancer {
    pub fn new() -> Balancer {
        Balancer {
            request: None,
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

        if total_mass > 0.0 {
            cx /= total_mass;
            cy /= total_mass;
        }

        // If the state has changed, we request to update the center of mass
        if (cx, cy) != canvas.center_of_mass() {
            self.request = request!(move |canvas| canvas.set_center_of_mass(cx, cy));
        }
    }

    fn execute(&mut self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request in Balancer");
                req(canvas.borrow_mut());
            },
            _ => {
                println!("{:?}", "No request to be executed by Balancer");
            }
        }
        self.request = None;
    }
}