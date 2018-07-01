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
    fn update(&mut self, canvas: &Canvas) {
        self.request = Some(request!(move |canvas| {
            let mut rect = Graphic::new("rect");
            rect.add_attr(ATTR!("width", canvas.dimensions().0));
            rect.add_attr(ATTR!("height", canvas.dimensions().1));
            rect.add_attr(ATTR!("fill", "#000000"));
            canvas.add_graphic(rect);
        }));
    }

    fn execute(&mut self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Painter");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Painter");
            }
        }
        self.request = None;
    }
}