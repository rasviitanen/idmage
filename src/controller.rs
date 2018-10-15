//! The controller allows agents to either update their current command, 
//! or execute their previous.
use canvas::Canvas;
use builder;
use agent::canvasagent::CanvasAgent;
use agent::request::Request;

pub struct Controller<'a> {
    canvas: &'a mut Canvas,
    modifiers: Vec<Box<CanvasAgent>>,
    observers: Vec<Box<CanvasAgent>>
}

impl<'a> Controller<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Controller<'a> {
        Controller {
            canvas,
            modifiers: Vec::new(),
            observers: Vec::new(),
        }
    }

    pub fn register_modifier(&mut self, agent: Box<CanvasAgent>) {
        self.modifiers.push(agent);
    }

    pub fn register_observer(&mut self, agent: Box<CanvasAgent>) {
        self.observers.push(agent);
    }
    
    pub fn tick(&mut self) {
        for i in 1..=2 {
            println!("########## Loop nr: {:?} ##########", i);
            let mut requests: Vec<Request> = Vec::new();

            for agent in &mut self.observers {
                match agent.update(self.canvas) {
                    Some(req) => { req.execute(self.canvas); },
                    _ => {println!("{:?}", "No request to execute")}
                };
            }
            
            for agent in &mut self.modifiers {
                match agent.update(self.canvas) {
                    Some(req) => { &requests.push(req); },
                    _ => {println!("{:?}", "No request to execute")}
                };
            }

            requests.sort();
            println!("Requests {:#?}", requests);
            if let Some(request) = requests.get(0) {
                request.execute(self.canvas);
            }

            for agent in &mut self.observers {
                match agent.update(self.canvas) {
                    Some(req) => { req.execute(self.canvas); },
                    _ => {println!("{:?}", "No request to execute")}
                };
            }

            // Display metrics here
            println!("----------- METRICS -----after--");
            println!("Metrics -> {:#?}", self.canvas.get_metrics());
            println!("Center of Mass -> {:#?}", self.canvas.center_of_mass());
            println!("--------------------------------");
        }
    }

    pub fn build(&self) -> String {
        builder::build(&self.canvas)
        
    }
}