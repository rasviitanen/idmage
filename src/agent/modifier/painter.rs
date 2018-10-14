use agent::canvasagent::CanvasAgent;
use canvas::Canvas;
use agent::request::Request;
use agent::request::ImpactMetricValue;
use metrics::Metric;

pub struct Painter {
    request: Option<Request>,
}

impl Painter {
    pub fn new() -> Painter {
        Painter {
            request: None,
        }
    }
}

impl CanvasAgent for Painter {
    fn update(&mut self, _: &Canvas) {
        let impact: ImpactMetricValue = 100;
        self.request = Some(request!(
            impact,
            move |canvas: &mut Canvas| {
                let (width, height) = canvas.dimensions();
                let background = canvas.profile()
                    .main_background(0.0, 0.0, width, height);
                canvas.get_metrics_mut().insert(Metric::BACKGROUND, 100);
                canvas.add_graphic(background);
            }
        ));
    }

    fn execute(&self, canvas: &mut Canvas) {
        match &self.request {
            Some(req) => {
                println!("{:?}", "Executing request for Painter");
                req.execute(canvas);
            },
            _ => {
                println!("{:?}", "No request to be executed by Painter");
            }
        }
    }
}