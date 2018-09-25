//! A profile for testing the path trace algorithm
use std::f64;

use profiles::profile::Profile;
use graphic::Graphic;


pub struct PathTraceProfile {
    pub name: String,
    pub slogan: String,
    pub font_family: Vec<String>,
    pub text_colors: Vec<String>,
    pub primary_colors: Vec<String>,
    pub background_colors: Vec<String>
}

impl PathTraceProfile {
    pub fn new() -> PathTraceProfile {
        let mut profile = PathTraceProfile {
            name: "Test".into(),
            slogan: "Perspective Projection Test".into(),
            font_family: Vec::new(),
            text_colors: Vec::new(),
            primary_colors: Vec::new(),
            background_colors: Vec::new(),
        };

        profile.text_colors.push("white".into());
        profile.text_colors.push("yellow".into());        
        profile.primary_colors.push("#0F0461".into());
        profile.primary_colors.push("#1E4FF2".into());
        profile.primary_colors.push("yellow".into());
        profile.background_colors.push("#429182".into());
        profile.font_family.push("Zilla Slab".into());
        profile.font_family.push("monospace".into());

        profile
    }
}

impl Profile for PathTraceProfile {

    fn font_family(&self) -> &Vec<String> { &self.font_family }

    fn text_colors(&self) -> &Vec<String> { &self.text_colors }

    fn primary_colors(&self) -> &Vec<String> { &self.primary_colors }

    fn background_colors(&self) -> &Vec<String> { &self.background_colors }

    fn main_background(&self, _x: f64, _y:f64, _width:f64, _height: f64) -> Graphic {
        let mut line: Graphic;

        let start_pos = (100.0, 100.0);
        let end_pos = (400.0, 400.0);
        
        line = Graphic::new("line");
        line.add_attr(ATTR!("x1", start_pos.0));
        line.add_attr(ATTR!("y1", start_pos.1));

        line.add_attr(ATTR!("x2", end_pos.0));
        line.add_attr(ATTR!("y2", end_pos.1));

        line.add_attr(ATTR!("style", "stroke:rgba(255,255,255,0.3);stroke-width:3"));

        line
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