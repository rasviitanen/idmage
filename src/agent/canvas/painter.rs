use agent::canvas::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::canvas::request::Request;
use agent::canvas::request::ImpactMetricValue;
use graphic::Graphic;

pub struct Painter {
    request: Option<Request>,
}

impl Painter {
    /// Test
    pub fn new() -> Painter {
        //! test1
        Painter {
            request: None,
        }
    }
}

impl CanvasAgent for Painter {
    fn update(&mut self, canvas: &Canvas) {
        let mut impact: ImpactMetricValue;
        impact = 100;
        self.request = Some(request!(
            impact,
            move |canvas| {
                let (width, height) = canvas.dimensions();
                let background = canvas.profile().main_background(0.0, 0.0, width, height);
                canvas.add_graphic(background);
            }
        ));
    }

    fn execute(&mut self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Painter");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Painter");
            }
        }
        self.request = None;
    }
}