//! Holds all the current information about the artwork and current status.
use graphic::Graphic;
use profiles::profile::Profile;
use profiles::pathtrace::PathTraceProfile;

pub struct Canvas {
    width: f64,
    height: f64,
    center_of_mass: (f64, f64, f64),
    graphics: Vec<Graphic>,
    profile: PathTraceProfile,
}

impl Canvas {
    pub fn new(width: f64, height:f64) -> Canvas {
        Canvas {
            width,
            height,
            center_of_mass: (width/2.0, height/2.0, 0.0),
            graphics: Vec::new(),
            profile: PathTraceProfile::new(),
        }
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn graphics(&self) -> &Vec<Graphic> {
        &self.graphics
    }

    pub fn add_graphic(&mut self, graphic: Graphic) {
        self.graphics.push(graphic);
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn set_center_of_mass(&mut self, cx: f64, cy: f64, cz: f64) {
        self.center_of_mass = (cx, cy, cz);
    }

    pub fn center_of_mass(&self) -> (f64, f64, f64) {
        self.center_of_mass
    }
}