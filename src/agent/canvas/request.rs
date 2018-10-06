use canvas::Canvas;

pub type ImpactMetricValue = u8;

macro_rules! request {
    ($impact:expr, $modification:expr) => {{
        Request { 
            impact: $impact,
            modification: Box::new($modification),
        }
    }};
}

pub struct Request {
    pub impact: ImpactMetricValue, // How big is my impact on the canvas, scale 0-100?
    pub modification: Box<Fn(&mut Canvas)>,
}

impl Request {
    pub fn execute(&self, canvas: &mut Canvas) {
        (self.modification)(canvas); // Execute the request on the canvas
    }
}