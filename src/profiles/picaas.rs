use std::f64;
use rand::prelude::*;

use profiles::profile::Profile;
use graphic::Graphic;


pub struct PicaasProfile {
    pub name: String,
    pub slogan: String,
    pub font_family: Vec<String>,
    pub text_colors: Vec<String>,
    pub primary_colors: Vec<String>,
    pub background_colors: Vec<String>
}

pub fn radial_gradient(id: &str, color1: &str, color2: &str, color3: &str, pos: (f64, f64), r: f64) -> Graphic {
    let mut gradient = Graphic::new("radialGradient");
    let mut stop1 = Graphic::new("stop");
    let mut stop2 = Graphic::new("stop");
    let mut stop3 = Graphic::new("stop");

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

fn tangent_line(p1: (f64, f64), p2: (f64, f64), r1: f64, r2: f64) -> (f64, f64) {
    let diff: (f64, f64) = (p2.0 - p1.0, p2.1 - p1.1);
    let distance: f64 = (diff.0.powi(2) + diff.1.powi(2)).sqrt();
    let dx = f64::cos(f64::consts::PI/2.0 
        + f64::atan((p2.1 - p1.1)/(p2.0 - p1.0)) + f64::asin((r2-r1)/distance));
    let dy = f64::sin(f64::consts::PI/2.0 
        + f64::atan((p2.1 - p1.1)/(p2.0 - p1.0)) + f64::asin((r2-r1)/distance));

    (dx, dy)
}

fn circle_network(color1: &str, color2: &str, color3: &str, v: &[((f64, f64), f64)]) -> Graphic {
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
    let _aass = "1000000000";

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
        circle1.center = (p1.0, p1.1, 0.0);
        circle1.add_weight(r1);


        circle2 = Graphic::new("circle");
        circle2.add_attr(ATTR!("cx", p2.0));
        circle2.add_attr(ATTR!("cy", p2.1));
        circle2.add_attr(ATTR!("r", r2));

        id = rng.gen_range(0, 100_000);

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

impl PicaasProfile {
    pub fn new() -> PicaasProfile {
        let mut profile = PicaasProfile {
            name: "Test".into(),
            slogan: "Preview".into(),
            font_family: Vec::new(),
            text_colors: Vec::new(),
            primary_colors: Vec::new(),
            background_colors: Vec::new(),
        };

        profile.text_colors.push("white".into());
        profile.text_colors.push("yellow".into());        
        profile.primary_colors.push("#0F2027".into());
        profile.primary_colors.push("#203A43".into());
        profile.primary_colors.push("#2C5364".into());
        profile.background_colors.push("#0F2027".into());
        profile.font_family.push("Zilla Slab".into());
        profile.font_family.push("monospace".into());

        profile
    }
}

impl Profile for PicaasProfile {

    fn font_family(&self) -> &Vec<String> { &self.font_family }

    fn text_colors(&self) -> &Vec<String> { &self.text_colors }

    fn primary_colors(&self) -> &Vec<String> { &self.primary_colors }

    fn background_colors(&self) -> &Vec<String> { &self.background_colors }

    fn main_background(&self, x: f64, y:f64, width:f64, height: f64) -> Graphic {
        let mut rng = thread_rng();
                    
        let mut position: (f64, f64);
        let mut radius: f64;

        let mut circle_list = Vec::new();
        
        // Create 3 to 7 random circles with size 1 to 6
        for _ in 0..rng.gen_range(3, 7) {
            position = (rng.gen_range(x, x + width), rng.gen_range(y, y + height));
            radius = rng.gen_range(0.5, 7.0);
            circle_list.push((position, radius));
        }

        // Connect the circles smoothly
        circle_network(
            &self.primary_colors[0], 
            &self.primary_colors[1], 
            &self.primary_colors[2], 
            &circle_list)
    }

    fn logo(&self, x: f64, y: f64, size: f64) -> Graphic {

        let mut text = Graphic::new("text");

        text.add_attr(ATTR!("font-size", size));
        text.add_attr(ATTR!("font-family", &self.font_family[0]));
        text.add_attr(ATTR!("font-weight", "Bold"));

        text.add_attr(ATTR!("x", x));
        text.add_attr(ATTR!("y", y));
        text.add_attr(ATTR!("text-anchor", "middle"));
        text.add_attr(ATTR!("fill", &self.text_colors[0]));

        text.add_text(&self.slogan);

        text
    }
}