use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;

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
        let mut cx: f64 = 0.0;
        let mut cy: f64 = 0.0;
        let mut total_mass = 0.0;
        for graphic in canvas.graphics() {
            let (graphic_cx, graphic_cy) = graphic.internal_center();
            cx += graphic_cx*graphic.weight();
            cy += graphic_cy*graphic.weight();
            total_mass += graphic.weight();
        }

        if total_mass > 0.0 {
            cx /= total_mass;
            cy /= total_mass;
        }

        // If the state has changed, we request to update the center of mass
        if (cx, cy) != canvas.center_of_mass() {
            self.request = Some(request!(move |cv| cv.set_center_of_mass(cx, cy)));
        }

    }

    fn execute(&mut self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Balancer");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Balancer");
            }
        }
        self.request = None;
    }
}