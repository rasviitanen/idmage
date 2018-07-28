use std::f64;

use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;
use graphic::Graphic;
use rand::prelude::*;


pub fn radial_gradient(id: &str, color1: &str, color2: &str, color3: &str, pos: (f64, f64), r: f64) -> Graphic {
    let mut gradient = Graphic::new("radialGradient");
    let mut stop1 = Graphic::new("stop");
    let mut stop2 = Graphic::new("stop");
    let mut stop3 = Graphic::new("stop");

    let mut animation = Graphic::new("animate");

    gradient.add_attr(ATTR!("id", id));
    gradient.add_attr(ATTR!("cx", pos.0));
    gradient.add_attr(ATTR!("cy", pos.1));
    gradient.add_attr(ATTR!("r", r));
    gradient.add_attr(ATTR!("gradientUnits", "userSpaceOnUse"));

    stop1.add_attr(ATTR!("offset", "10%"));
    stop1.add_attr(ATTR!("stop-color", color1));

    stop2.add_attr(ATTR!("offset", "50%"));
    stop2.add_attr(ATTR!("stop-color", color2));

    stop3.add_attr(ATTR!("offset", "100%"));
    stop3.add_attr(ATTR!("stop-color", color3));

    gradient.add_child(stop1);
    gradient.add_child(stop2);
    gradient.add_child(stop3);
    
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

fn circle_network(color1: &str, color2: &str, color3: &str, v: &Vec<((f64, f64), f64)>) -> Graphic {
    let mut i = 0;
    let mut v_iter = v.iter();
    let mut c1: &((f64, f64), f64);
    let mut c2: &((f64, f64), f64);

    let mut parent = Graphic::new("g");
    let mut rng = thread_rng();

    let mut g: Graphic;
    let mut circle1: Graphic;
    let mut circle2: Graphic;
    let mut id: i32;

    let mut p1: (f64, f64);
    let mut p2: (f64, f64);
    let mut r1: f64;
    let mut r2: f64;

    let mut angle: (f64, f64);

    let mut polygon: Graphic;

    c1 = v_iter.next().unwrap();
    while v.len() - 1 > i {

        c2 = v_iter.next().unwrap();

        p1 = c1.0;
        r1 = c1.1;
        p2 = c2.0;
        r2 = c2.1;

        angle = tangent_line(p1, p2, r1, r2);

        g = Graphic::new("g");

        circle1 = Graphic::new("circle");
        circle1.add_attr(ATTR!("cx", p1.0));
        circle1.add_attr(ATTR!("cy", p1.1));
        circle1.add_attr(ATTR!("r", r1));


        circle2 = Graphic::new("circle");
        circle2.add_attr(ATTR!("cx", p2.0));
        circle2.add_attr(ATTR!("cy", p2.1));
        circle2.add_attr(ATTR!("r", r2));

        id = rng.gen_range(0, 100000);

        let diff: (f64, f64) = (p2.0 - p1.0, p2.1 - p1.1);
        let radius: f64 = (diff.0.powi(2) + diff.1.powi(2)).sqrt();

        // Alternate the color
        if i%2==1 {
            g.add_child(radial_gradient(&format!("grad{}", id), color1, color2, color3, p2, radius));
        } else {
            g.add_child(radial_gradient(&format!("grad{}", id), color3, color2, color1, p2, radius));
        }
        g.add_attr(ATTR!("fill", format!("url(#grad{})", id)));

        polygon = Graphic::new("polygon");
        polygon.add_attr(format!("points=\"{p1_x},{p1_y} {p2_x},{p2_y} {p3_x},{p3_y} {p4_x},{p4_y}\"", 
                p1_x = p1.0 + r1 * angle.0,
                p1_y = p1.1 + r1 * angle.1,
                p2_x = p2.0 + r2 * angle.0,
                p2_y = p2.1 + r2 * angle.1,
                p3_x = p2.0 - r2 * angle.0,
                p3_y = p2.1 - r2 * angle.1,
                p4_x = p1.0 - r1 * angle.0,
                p4_y = p1.1 - r1 * angle.1
            ));

        g.add_child(circle1);
        g.add_child(polygon);
        g.add_child(circle2);

        parent.add_child(g);
        
        c1 = c2;

        i += 1;
        
    }
    parent
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
            let mut rng = thread_rng();
            
            let mut background = Graphic::new("rect");

            let mut group = Graphic::new("g");
            let mut circle_list = Vec::new();

            let mut position: (f64, f64);
            let mut radius: f64;

            background.add_attr(ATTR!("x", 0));
            background.add_attr(ATTR!("y", 0));
            background.add_attr(ATTR!("width", width));
            background.add_attr(ATTR!("height", height));

            background.add_attr(ATTR!("fill", &canvas.profile().background_colors[0]));

            let mut text = Graphic::new("text");

            text.add_attr(ATTR!("font-size", 112));
            text.add_attr(ATTR!("font-family", &canvas.profile().font_family[0]));
            text.add_attr(ATTR!("font-weight", "Bold"));

            text.add_text(&canvas.profile().name);
            text.add_attr(ATTR!("x", width/4.0));
            text.add_attr(ATTR!("y", 2.0*height/3.0));
            text.add_attr(ATTR!("text-anchor", "middle"));
            text.add_attr(ATTR!("fill", &canvas.profile().text_colors[0]));

            let mut text_small = Graphic::new("text");

            text_small.add_attr(ATTR!("font-size", 32.0));
            text_small.add_attr(ATTR!("font-family", &canvas.profile().font_family[0]));
            text_small.add_text(&canvas.profile().slogan);
            text_small.add_attr(ATTR!("x", width/4.0));
            text_small.add_attr(ATTR!("y", 2.0*height/3.0 + 44.0));
            text_small.add_attr(ATTR!("text-anchor", "middle"));

            text_small.add_attr(ATTR!("fill", &canvas.profile().text_colors[1]));

            for _ in 0..rng.gen_range(3, 7) {
                position = (rng.gen_range(width/4.0, width + 100.0), rng.gen_range(0.0, height + 100.0));
                radius = rng.gen_range(1.0, 6.0);
                circle_list.push((position, radius));
            } 

            group.add_child(circle_network(
                &canvas.profile().primary_colors[0], 
                &canvas.profile().primary_colors[1], 
                &canvas.profile().primary_colors[2], 
                &circle_list));

            canvas.add_graphic(background);
            canvas.add_graphic(group);
            canvas.add_graphic(text);
            canvas.add_graphic(text_small);
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