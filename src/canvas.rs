//! Holds all the current information about the artwork and current status.
use graphic::Graphic;
use profiles::profile::Profile;
use profiles::picaas::PicaasProfile;
use math::projection::Camera;
use std::collections::HashMap;
use metrics::Metric;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub struct Canvas {
    width: f64,
    height: f64,
    camera: Option<Camera>,
    center_of_mass: (f64, f64, f64),
    graphics: Vec<Graphic>,
    profile: PicaasProfile,
    metrics: HashMap<Metric, u8>,
}

impl Canvas {
    pub fn new(width: f64, height:f64) -> Canvas {
        Canvas {
            width,
            height,
            camera: None,
            center_of_mass: (width/2.0, height/2.0, 0.0),
            graphics: Vec::new(),
            profile: PicaasProfile::new(),
            metrics: map!(
                Metric::BALANCE => 0,
                Metric::FLOW => 0
            )
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = Some(camera);
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