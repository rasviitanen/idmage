use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;
use graphic::Graphic;
use std::f64::consts;
use rand::Rng;

pub fn log_spiral(t: f64, cx: f64, cy: f64) -> (f64, f64) {
    let x = 0.1*consts::E.powf(0.04*t)* t.cos();
    let y = 0.1*consts::E.powf(0.04*t)* t.sin();

    (x + cx, y + cy)
}

pub fn log_spiral_2(t: f64, cx: f64, cy: f64) -> (f64, f64) {
    let x = 0.05*consts::E.powf(0.06*t)* t.cos();
    let y = 0.05*consts::E.powf(0.06*t)* t.sin();
    
    (x + cx, y + cy)
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
        self.request = Some(request!(move |canvas| {
            let (width, height) = canvas.dimensions();
            let mut group = Graphic::new("g");
            let mut coord: (f64, f64);
            let mut circle: Graphic;
            let mut spiral_selection: i32;
            spiral_selection = rand::thread_rng().gen_range(0, 2);

            for t in 120..300 {
                circle = Graphic::new("circle");
                
                if spiral_selection == 0 {
                    coord = log_spiral(t as f64, width/2.0, height/2.0);
                } else {
                    coord = log_spiral_2(t as f64, width/2.0, height/2.0);
                }
                
                circle.add_attr(ATTR!("r", 3));
                circle.add_attr(ATTR!("cx", coord.0));
                circle.add_attr(ATTR!("cy", coord.1));
                circle.add_attr(ATTR!("fill", RGBA!(120, (coord.1/width)*255.0, (coord.0/height)*255.0, 1)));
                group.add_child(circle);
                
            }
            canvas.add_graphic(group);
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