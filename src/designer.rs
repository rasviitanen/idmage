use super::architect::Blueprint;
use super::profiler::Color;
use super::graphic::Graphic;

pub struct Designer{
    focal_point: (f64, f64),
}

impl Designer {
    pub fn new() -> Designer {
        Designer {
            focal_point: (0.0, 0.0),
        }
    }

    pub fn start(&self, bp: &mut Blueprint) {
        println!("In Designer start");
        // Implement control-flow
    }

    pub fn pick_color(&self, bp: &mut Blueprint)  {

    }

    pub fn pick_location(&self, bp: &mut Blueprint) {

    }

    pub fn pick_size(&self, bp: &mut Blueprint) {

    }

}