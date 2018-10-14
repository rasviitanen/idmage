//! Holds all the current information about the artwork and current status.
use graphic::Graphic;
use profiles::profile::Profile;
use profiles::picaas::PicaasProfile;
use math::projection::Camera;
use std::collections::HashMap;
use metrics::Metric;
use math::helpers::Coordinate;

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
    center_of_mass: Coordinate,
    graphics: Vec<Graphic>,
    profile: PicaasProfile,
    metrics: HashMap<Metric, u8>,
}

impl Canvas {
    pub fn new(width: f64, height:f64) -> Canvas {
        let mut canvas = Canvas {
            width,
            height,
            camera: None,
            center_of_mass: Coordinate{x: width/2.0, y: height/2.0, z: 0.0},
            graphics: Vec::new(),
            profile: PicaasProfile::new(),
            metrics: map!(
                Metric::BACKGROUND => 0,
                Metric::FOREGROUND => 0
            )
        };

        let mut base: Graphic = Graphic::new("rect");
        base.add_attr(ATTR!("x", 0));
        base.add_attr(ATTR!("y", 0));
        base.add_attr(ATTR!("width", width));
        base.add_attr(ATTR!("height", height));
        base.add_attr(ATTR!("fill", canvas.profile.background_colors()[0]));
        canvas.add_graphic(base);

        canvas
    }

    pub fn get_metrics(&self) -> &HashMap<Metric, u8> {
        &self.metrics
    }

    pub fn get_metrics_mut(&mut self) -> &mut HashMap<Metric, u8> {
        &mut self.metrics
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

    pub fn set_center_of_mass(&mut self, center: &Coordinate) {
        self.center_of_mass.x = center.x;
        self.center_of_mass.y = center.y;
        self.center_of_mass.z = center.z;
    }

    pub fn center_of_mass(&self) -> &Coordinate {
        &self.center_of_mass
    }
}