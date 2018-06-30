use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;
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

impl CanvasAgent for Painter {
    fn update(&mut self, _: &Canvas) {
        self.request = Some(request!(move |cv| {
            let mut circle = Graphic::new("circle");
            circle.add_attr("cx=\"500\"");
            circle.add_attr("cy=\"500\"");
            circle.add_attr("r=\"120\"");
            
            cv.add_graphic(circle);
        }));
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