use agent::agent::Agent;
use agent::tiler::Tiler;
use canvas::Canvas;
use builder;

pub struct Controller<'a> {
    canvas: &'a mut Canvas,
    agents: Vec<Box<Agent>>,
}

impl<'a> Controller<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Controller {
        Controller {
            canvas,
            agents: vec!(Box::new(Tiler::new())),
        }
    }

    fn tick(&mut self) {
        for agent in &self.agents {
            agent.update(self.canvas);
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