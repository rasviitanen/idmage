#[derive(Debug)]
pub struct Graphic<'a> {
    element: String,
    weight: f64,
    attributes: Vec<String>,
    children: Vec<&'a Graphic<'a>>
}

impl<'a> Graphic<'a> {
    pub fn new(element: String) -> Graphic<'a> {
        Graphic {
            element,
            weight: 0.0,
            attributes: Vec::new(),
            children: Vec::new()
        }
    }
    
    pub fn add_child(&mut self, graphic: &'a Graphic) {
        &self.children.push(graphic);
    }
}