//! The controller allows agents to either update their current command, 
//! or execute their previous.
use canvas::Canvas;
use builder;
use agent::canvasagent::CanvasAgent;

pub struct Controller<'a> {
    canvas: &'a mut Canvas,
    agents: Vec<Box<CanvasAgent>>,
}

impl<'a> Controller<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Controller<'a> {
        Controller {
            canvas,
            agents: Vec::new(),
        }
    }

    pub fn register_agent(&mut self, agent: Box<CanvasAgent>) {
        self.agents.push(agent);
    }
    
    pub fn tick(&mut self) {
        for _ in 0..1 {
            for agent in &mut self.agents {
                println!("{:?}", self.canvas.get_metrics());
                agent.update(self.canvas);
                agent.execute(self.canvas);
            }
        }
    }

    pub fn build(&self) -> String {
        builder::build(&self.canvas)
        
    }
}