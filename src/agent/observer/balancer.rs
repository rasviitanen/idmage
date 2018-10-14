use math::helpers::Coordinate;
use agent::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::request::Request;

pub struct Balancer {
    request: Option<Request>,
}

impl Balancer {
    pub fn new() -> Balancer {
        Balancer {
            request: None,
        }
    }
}

impl CanvasAgent for Balancer {
    fn update(&mut self, canvas: &Canvas) {
        // Calculate center of mass
        let mut center: Coordinate = Coordinate{x: 0.0, y: 0.0, z: 0.0};
    
        let mut total_mass = 0.0;

        for graphic in canvas.graphics() {
            let graphic_center: &Coordinate = graphic.get_center();
            center.x += graphic_center.x*graphic.weight();
            center.y += graphic_center.y*graphic.weight();
            center.z += graphic_center.z*graphic.weight();

            total_mass += graphic.weight();
        }
        
        if total_mass > 0.0 {
            center.x /= total_mass;
            center.y /= total_mass;
            center.z /= total_mass;
        }

        // If the state has changed, we request to update the center of mass
        if center != *(canvas.center_of_mass()) {
            self.request = Some(request!(
                100,
                move |canvas: &mut Canvas| {
                    canvas.set_center_of_mass(&center);
                }
            ));
        }

    }

    fn execute(&self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Balancer");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Balancer");
            }
        }
        println!("{:?}", canvas.center_of_mass());
    }
}