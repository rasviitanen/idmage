use canvas::Canvas;
use builder;
use agent::canvas::canvasagent::CanvasAgent;

pub struct Controller<'a> {
    canvas: &'a Canvas<'a>,
    agents: Vec<Box<CanvasAgent>>
}

impl<'a> Controller<'a> {
    pub fn new(canvas: &'a mut Canvas<'a>) -> Controller<'a> {
        Controller {
            canvas,
            agents: Vec::new(),
        }
    }

    pub fn register_agent(&mut self, agent: Box<CanvasAgent>) {
        self.agents.push(agent);
    }
    
    pub fn tick(&mut self) {
        for agent in &mut self.agents {
            agent.update(self.canvas);
        }
    }

    pub fn build(&self) -> String {
        let out = builder::build(&self.canvas);
        out
    }
}