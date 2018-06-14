use agent::agent::Agent;
use agent::balancer::Balancer;
use agent::tiler::Tiler;
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
            agents: vec!(Box::new(
                Balancer::new()), 
                Box::new(Tiler::new())
            ),
        }
    }

    fn tick(&mut self) {
        for agent in &mut self.agents {
            agent.update(self.canvas);
        }
    }

    pub fn build(&mut self) -> String {
        for _ in 0..5 {
            &self.tick();
        }
        println!("{:#?}", self.canvas);
        let out = builder::build(&self.canvas);
        out
    }
}