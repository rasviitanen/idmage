//! A profile for testing the path trace algorithm
use std::f64;
use rand::prelude::*;

use profiles::profile::Profile;
use graphic::Graphic;
use math::projection;
use graphics::nurbs;
use graphics::nurbs::Anchor;


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
        profile.background_colors.push("#000000".into());
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

    fn main_background(&self, x: f64, y:f64, width:f64, height: f64) -> Graphic {

        let position = (width/2.0, height/2.0, 0.0);
        let orientation = (0.0011, 0.0007, 0.0);
        let projection_plane = (width/2.0, height/2.0, 1.0);
        let camera = projection::Camera::new(
            position, orientation, projection_plane);

        let mut lines = Graphic::new("g");
        let mut line: Graphic;
        let mut line2: Graphic;

        let mut start_pos;
        let mut end_pos;
        
        for i in -12..=12 {
            // Create lines

            start_pos = projection::project_3d_point_on_2d_surface(&camera, 
                (width/2.0 + (i as f64)*100.0, -1000.0, 5.0));
            end_pos = projection::project_3d_point_on_2d_surface(&camera, 
                (width/2.0 + (i as f64)*100.0, 1000.0, 5.0));

            line = Graphic::new("line");
            line.add_attr(ATTR!("x1", start_pos.0));
            line.add_attr(ATTR!("y1", start_pos.1));

            line.add_attr(ATTR!("x2", end_pos.0));
            line.add_attr(ATTR!("y2", end_pos.1));

            line.add_attr(ATTR!("style", "stroke:rgba(255,255,255,0.3);stroke-width:3"));

            lines.add_child(line);

            start_pos = projection::project_3d_point_on_2d_surface(&camera, 
                (width/2.0 - 1000.0, height/2.0 + (i as f64)*100.0, 5.0));
            end_pos = projection::project_3d_point_on_2d_surface(&camera, 
                (width/2.0 + 1000.0, height/2.0 + (i as f64)*100.0, 5.0));
            // Create lines
            line2 = Graphic::new("line");
            line2.add_attr(ATTR!("x1", start_pos.0));
            line2.add_attr(ATTR!("y1", start_pos.1));

            line2.add_attr(ATTR!("x2", end_pos.0));
            line2.add_attr(ATTR!("y2", end_pos.1));

            line2.add_attr(ATTR!("style", "stroke:rgba(255,255,255, 0.3);stroke-width:3"));

            lines.add_child(line2);
        }
        

        lines
    }

    fn logo(&self, x: f64, y: f64, size: f64) -> Graphic {

        let position = (1280.0/2.0, 420.0/2.0, 0.0);
        let orientation = (0.0013, 0.0007, 0.0);
        let projection_plane = (1280.0/2.0, 420.0/2.0, 1.0);
        let camera = projection::Camera::new(
            position, orientation, projection_plane);
            
        let bezier = nurbs::nurbs_surface(&camera, &vec!(
                bezier!('M', (200.0, 200.0, 4.0)),
                bezier!('C', (200.0, 250.0, 4.0)),
                bezier!('C', (400.0, 350.0, 2.0), (300.0, 400.0, 2.0)),
                bezier!('S', (800.0, 500.0, 2.0), (800.0, 200.0, 2.0)),
                bezier!('S', (200.0, 0.0, 4.0), (200.0, 200.0, 4.0)),
            ));

        bezier
    }
}