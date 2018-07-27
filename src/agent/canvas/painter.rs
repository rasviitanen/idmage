use std::f64;

use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;
use graphic::Graphic;
use rand::prelude::*;


pub fn radial_gradient(id: &str, color1: &str, color2: &str, pos: (f64, f64), r: f64) -> Graphic {
    let mut gradient = Graphic::new("radialGradient");
    let mut stop1 = Graphic::new("stop");
    let mut stop2 = Graphic::new("stop");
    
    gradient.add_attr(ATTR!("id", id));
    gradient.add_attr(ATTR!("cx", pos.0));
    gradient.add_attr(ATTR!("cy", pos.1));
    gradient.add_attr(ATTR!("r", r));
    gradient.add_attr(ATTR!("gradientUnits", "userSpaceOnUse"));

    stop1.add_attr(ATTR!("offset", "10%"));
    stop1.add_attr(ATTR!("stop-color", color1));

    stop2.add_attr(ATTR!("offset", "950%"));
    stop2.add_attr(ATTR!("stop-color", color2));

    gradient.add_child(stop1);
    gradient.add_child(stop2);
    
    gradient
}

pub struct Painter {
    request: Option<Request>,
}

fn tangent_line(p1: (f64, f64), p2: (f64, f64), r1: f64, r2: f64) -> (f64, f64) {
    let diff: (f64, f64) = (p2.0 - p1.0, p2.1 - p1.1);
    let distance: f64 = (diff.0.powi(2) + diff.1.powi(2)).sqrt();
    let dx = f64::cos(f64::consts::PI/2.0 
        + f64::atan((p2.1 - p1.1)/(p2.0 - p1.0)) + f64::asin((r2-r1)/distance));
    let dy = f64::sin(f64::consts::PI/2.0 
        + f64::atan((p2.1 - p1.1)/(p2.0 - p1.0)) + f64::asin((r2-r1)/distance));

    (dx, dy)
}

fn circle_polygon_joiner(n: i32, p1: (f64, f64), p2: (f64, f64), r1: f64, r2: f64) -> Graphic {
    let (dx, dy) = tangent_line(p1, p2, r1, r2);
    let mut g = Graphic::new("g");

    let mut circle1 = Graphic::new("circle");
    circle1.add_attr(ATTR!("cx", p1.0));
    circle1.add_attr(ATTR!("cy", p1.1));
    circle1.add_attr(ATTR!("r", r1));

    let mut circle2 = Graphic::new("circle");
    circle2.add_attr(ATTR!("cx", p2.0));
    circle2.add_attr(ATTR!("cy", p2.1));
    circle2.add_attr(ATTR!("r", r2));

    let mut rng = thread_rng();
    let id = rng.gen_range(0, 100000);

    let diff: (f64, f64) = (p2.0 - p1.0, p2.1 - p1.1);
    let radius: f64 = (diff.0.powi(2) + diff.1.powi(2)).sqrt();

    if n%2 == 0 {
        g.add_child(radial_gradient(&format!("grad{}", id), "red", "blue", p2, radius));
    } else {
        g.add_child(radial_gradient(&format!("grad{}", id), "blue", "red", p2, radius));
    }

    let mut polygon = Graphic::new("polygon");
    polygon.add_attr(format!("points=\"{p1_x},{p1_y} {p2_x},{p2_y} {p3_x},{p3_y} {p4_x},{p4_y}\"", 
            p1_x = p1.0 + r1 * dx,
            p1_y = p1.1 + r1 * dy,
            p2_x = p2.0 + r2 * dx,
            p2_y = p2.1 + r2 * dy,
            p3_x = p2.0 - r2 * dx,
            p3_y = p2.1 - r2 * dy,
            p4_x = p1.0 - r1 * dx,
            p4_y = p1.1 - r1 * dy
        ));
    g.add_attr(ATTR!("fill", format!("url(#grad{})", id)));

    g.add_child(circle1);
    g.add_child(polygon);
    g.add_child(circle2);

    g
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
        let p1 = (75.0, 75.0);
        let r1 = 40.0;

        let p2 = (500.0, 730.0);
        let r2 = 15.0;

        let p3 = (700.0, 470.0);
        let r3 = 5.0;

        let p4 = (100.0, 600.0);
        let r4 = 20.0;

        self.request = Some(request!(move |canvas| {
            let mut group = Graphic::new("g");

            group.add_child(circle_polygon_joiner(0, p1, p2, r1, r2));
            group.add_child(circle_polygon_joiner(1, p2, p3, r2, r3));
            group.add_child(circle_polygon_joiner(2, p3, p4, r3, r4));

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