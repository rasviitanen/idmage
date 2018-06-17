use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;

pub struct Tiler {
    request: Option<Request>,
}

impl Tiler {
    pub fn new() -> Tiler {
        Tiler {
            request: None,
        }
    }
}

impl CanvasAgent for Tiler {
    fn update(&mut self, canvas: &Canvas) {
        let (cx, cy) = canvas.center_of_mass();
        if canvas.tiles_borrow().len() == 0 {
            self.request = Some(request!(move |canvas| {
                canvas.add_tile((cx-100.0, cy-100.0), (cx+100.0, cy+100.0))
            }));
        }
    }

    fn execute(&mut self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Tiler");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Tiler");
            }
        }
        self.request = None;
    }
}