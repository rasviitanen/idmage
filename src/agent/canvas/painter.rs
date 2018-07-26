use std::f64;

use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;
use graphic::Graphic;

pub struct Painter {
    request: Option<Request>,
}

fn tangent_line(p1: (f64, f64), p2: (f64, f64), r1: f64, r2: f64) -> (f64, f64) {
    let diff: (f64, f64) = (p2.0 - p1.0, p2.1 - p1.1);
    let distance: f64 = (diff.0.powi(2) + diff.1.powi(2)).sqrt();
    println!("{:?}", distance);    
    let dx = f64::cos(f64::consts::PI/2.0 
        + f64::atan((p2.1 - p1.1)/(p2.0 - p1.0)) + f64::asin((r2-r1)/distance));
    let dy = f64::sin(f64::consts::PI/2.0 
        + f64::atan((p2.1 - p1.1)/(p2.0 - p1.0)) + f64::asin((r2-r1)/distance));

    (dx, dy)
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
        let (dx, dy) = tangent_line((100.0, 100.0), (600.0, 600.0), 40.0, 100.0);
        self.request = Some(request!(move |canvas| {
            let mut group = Graphic::new("g");
            
            let mut circle_small = Graphic::new("circle");
            circle_small.add_attr(ATTR!("cx", 100));
            circle_small.add_attr(ATTR!("cy", 100));
            circle_small.add_attr(ATTR!("r", 40));
            circle_small.add_attr(ATTR!("fill", "red"));

            let mut circle_big = Graphic::new("circle");
            circle_big.add_attr(ATTR!("cx", 600));
            circle_big.add_attr(ATTR!("cy", 600));
            circle_big.add_attr(ATTR!("r", 100));
            circle_big.add_attr(ATTR!("fill", "blue"));

            let mut polygon = Graphic::new("polygon");
            polygon.add_attr(format!("points=\"{p1_x},{p1_y} {p2_x},{p2_y} {p3_x},{p3_y} {p4_x},{p4_y}\"", 
                    p1_x=100.0+40.0*dx,
                    p1_y=100.0+40.0*dy,
                    p2_x=600.0+100.0*dx,
                    p2_y=600.0+100.0*dy,
                    p3_x=600.0-100.0*dx,
                    p3_y=600.0-100.0*dy,
                    p4_x=100.0-40.0*dx,
                    p4_y=100.0-40.0*dy
                ));
            polygon.add_attr(ATTR!("style", "fill:url(#MyGrad)"));

            group.add_child(circle_small);
            group.add_child(polygon);
            group.add_child(circle_big);
            canvas.add_graphic(group);
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