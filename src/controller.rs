use agent::agent::Agent;
use agent::tiler::Tiler;
use agent::balancer::Balancer;
use canvas::Canvas;
use builder;

pub struct Controller<'a> {
    pub canvas: &'a mut Canvas<'a>,
    agents: Vec<Box<Agent>>,    
}

impl<'a> Controller<'a> {
    pub fn new(canvas: &'a mut Canvas<'a>) -> Controller<'a> {
        Controller {
            canvas: canvas,
            agents: vec!(Box::new(Tiler::new()), Box::new(Balancer::new())),
        }
    }

    fn tick(&mut self) {
        for agent in &mut self.agents {
            agent.update(self.canvas);
            agent.execute();
        }
    }

    pub fn build(&mut self) -> String {
        for _ in 0..5 {
            &self.tick();
        }
        let out = builder::build(&self.canvas);
        out
    }
}