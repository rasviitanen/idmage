use graphic::Graphic;
use profile::Profile;

pub struct Canvas {
    width: f64,
    height: f64,
    aspect_ratio: f64,
    center_of_mass: (f64, f64),
    graphics: Vec<Graphic>,
    profile: Profile,
}

impl Canvas {
    pub fn new(width: f64, height:f64) -> Canvas {
        Canvas {
            width,
            height,
            aspect_ratio: width/height,
            center_of_mass: (width/2.0, height/2.0),
            graphics: Vec::new(),
            profile: Profile::new(),
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

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn set_center_of_mass(&mut self, cx: f64, cy: f64) {
        self.center_of_mass = (cx, cy);
    }

    pub fn center_of_mass(&self) -> (f64, f64) {
        self.center_of_mass
    }
}