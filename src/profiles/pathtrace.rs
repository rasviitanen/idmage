//! A profile for testing the path trace algorithm
use std::f64;
use rand::prelude::*;

use profiles::profile::Profile;
use graphic::Graphic;
use math::projection;

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
            slogan: "Path Tracing Test".into(),
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
        profile.background_colors.push("#666666".into());
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

    /// Creates a few random circles of medium size
    fn main_background(&self, x: f64, y:f64, width:f64, height: f64) -> Graphic {
        let mut position: (f64, f64, f64);

        let camera = projection::Camera::new((width/2.0, height/2.0, 0.0), (0.3, 3.0, 0.0), (width/2.0, height/2.0, 200.0));

        let mut lines = Graphic::new("g");
        let mut line: Graphic;
        let mut line2: Graphic;

        let mut start_pos;
        let mut end_pos;
        
        for i in -1..=1 {
            // Create lines

            start_pos = projection::project_3d_point_on_2d_surface(&camera, (width/2.0 + (i as f64)*100.0, 0.0, 500.0));
            end_pos = projection::project_3d_point_on_2d_surface(&camera, (width/2.0 + (i as f64)*100.0, height, 500.0));

            line = Graphic::new("line");
            line.add_attr(ATTR!("x1", start_pos.0));
            line.add_attr(ATTR!("y1", start_pos.1));

            line.add_attr(ATTR!("x2", end_pos.0));
            line.add_attr(ATTR!("y2", end_pos.1));

            line.add_attr(ATTR!("style", "stroke:rgb(255,0,0);stroke-width:2"));

            lines.add_child(line);

            start_pos = projection::project_3d_point_on_2d_surface(&camera, (width/2.0 - 300.0, height/2.0 + (i as f64)*100.0, 500.0));
            end_pos = projection::project_3d_point_on_2d_surface(&camera, (width/2.0 + 300.0, height/2.0 + (i as f64)*100.0, 500.0));
            // Create lines
            line2 = Graphic::new("line");
            line2.add_attr(ATTR!("x1", start_pos.0));
            line2.add_attr(ATTR!("y1", start_pos.1));

            line2.add_attr(ATTR!("x2", end_pos.0));
            line2.add_attr(ATTR!("y2", end_pos.1));

            line2.add_attr(ATTR!("style", "stroke:rgb(255,0,0);stroke-width:2"));

            lines.add_child(line2);
        }
        

        lines
    }

    /// A basic string containg "Path Tracing Text" in Zilla Slab
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