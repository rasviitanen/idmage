use agent::agent::Agent;
use canvas::Canvas;
use agent::request::Request;

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

impl Agent for Painter {
    fn update(&mut self, canvas: &Canvas) {
        for tile in canvas.tiles() {
            self.request = Some(request!(move |canvas| canvas.add_graphic_to_tile(cx, cy)));
        }
    }

    fn execute(&mut self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Balancer");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Balancer");
            }
        }
        self.request = None;
    }
}