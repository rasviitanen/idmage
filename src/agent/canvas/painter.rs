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
            let (width, height) = canvas.dimensions();
            let mut background = Graphic::new("rect");
            let mut group = Graphic::new("g");

            background.add_attr(ATTR!("x", 0));
            background.add_attr(ATTR!("y", 0));
            background.add_attr(ATTR!("width", width));
            background.add_attr(ATTR!("height", height));

            background.add_attr(ATTR!("fill", &canvas.profile().background_colors()[0]));

            let mut text = Graphic::new("text");

            text.add_attr(ATTR!("font-size", 64));
            text.add_attr(ATTR!("font-family", &canvas.profile().font_family()[0]));

            group.add_child(canvas.profile().main_background(0.0, 0.0, width, height));
            group.add_child(canvas.profile().logo(width/2.0, height/2.0, 84.0));
            canvas.add_graphic(background);
            canvas.add_graphic(group);
            canvas.add_graphic(text);
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