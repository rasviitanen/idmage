use super::architect::Blueprint;
use super::instructions::LayoutInstruction;
use super::instructions::PaintInstruction;
use super::distributions;
use instructions;

pub struct Layout {
    paint_instructions: Vec<PaintInstruction>,
}

impl Layout {
    pub fn new() -> Layout {
        Layout{ paint_instructions: Vec::new()}
    }

    pub fn draw(&mut self, bp: &mut Blueprint) {
        for graphic in bp.get_graphics() {
            let content = "";
            graphic.set_attribute("cx", "100");
            graphic.set_attribute("cy", "100");
            graphic.set_attribute("r", "200");
            &self.paint_instructions.push(
                instructions::PaintInstruction::
                new(graphic.get_element(), graphic.get_attributes_as_string(), content));
        }
    }

    pub fn get_paint_instructions(&self) -> &Vec<PaintInstruction> {
        &self.paint_instructions
    }
}
