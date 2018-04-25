pub enum LayoutInstruction {
    Discrete,
    Single,
    Isolated,
    Heading,
    Tailing,
    Symmetric,
    RuleOfThirds,
    ChaoticSpray,
    LogSpiralSpray,
    CircleSpray,
}

pub struct PaintInstruction {
    element: String,
    attributes: String,
    content: String
}

impl PaintInstruction {
    pub fn new(element: &str, attributes: String, content: &str) -> PaintInstruction {
        PaintInstruction {
            element: element.to_string(),
            attributes,
            content: content.to_string()
        }
    }
    pub fn get_element(&self) -> &String {&self.element }
    pub fn get_attributes(&self) -> &String {&self.attributes}
    pub fn get_content(&self) -> &String {&self.content }
}