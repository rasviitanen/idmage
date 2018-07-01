use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;
use graphic::Graphic;
use std::f64::consts;

pub fn log_spiral(t: f64) -> (f64, f64) {
    let mut x = 0.1*consts::E.powf(0.04*t)* t.cos();
    let mut y = 0.1*consts::E.powf(0.04*t)* t.sin();
 
    x = x;
    y = y;

    (x + 1920.0/2.0, y + 1080.0/2.0)
}

pub struct Spiral {
    request: Option<Request>,
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            request: None,
        }
    }
}

impl CanvasAgent for Spiral {
    fn update(&mut self, canvas: &Canvas) {
        self.request = Some(request!(move |cv| {
            
            let mut group = Graphic::new("g");
            let mut coord: (f64, f64);
            let mut circle: Graphic;
            let mut animation: Graphic;


            animation = Graphic::new("animateTransform");
            animation.add_attr(ATTR!("attributeType", "xml"));
            animation.add_attr(ATTR!("attributeName", "transform"));
            animation.add_attr(ATTR!("type", "rotate"));
            animation.add_attr(ATTR!("from", "0 960 540"));
            animation.add_attr(ATTR!("to", "360 960 540"));
            animation.add_attr(ATTR!("dur", "10s"));
            animation.add_attr(ATTR!("repeatCount", "indefinite"));
            group.add_child(animation);

            for t in 120..300 {
                circle = Graphic::new("circle");
                coord = log_spiral(t as f64);
                circle.add_attr(ATTR!("r", 3));
                circle.add_attr(ATTR!("cx", coord.0));
                circle.add_attr(ATTR!("cy", coord.1));
                circle.add_attr(ATTR!("fill", RGBA!(120, (coord.1/1080.0)*255.0, (coord.0/1920.0)*255.0, 1)));
                group.add_child(circle);
            }
            cv.add_graphic(group);
        }));
    }

    fn execute(&mut self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Spiral");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Spiral");
            }
        }
        self.request = None;
    }
}