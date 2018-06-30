#[derive(Debug)]
pub struct Graphic<'a> {
    element: String,
    weight: f64,
    center: (f64, f64),
    attributes: Vec<String>,
    children: Vec<&'a Graphic<'a>>
}

impl<'a> Graphic<'a> {
    pub fn new(element: &str) -> Graphic<'a> {
        Graphic {
            element: element.to_string(),
            weight: 0.0,
            center: (0.0, 0.0),
            attributes: Vec::new(),
            children: Vec::new()
        }
    }

    pub fn add_weight(&mut self, weight: f64) {
        self.weight += weight;
    }

    pub fn weight(&self) -> (f64) {
        self.weight
    }

    pub fn internal_center(&self) -> (f64, f64) {
        self.center
    }

    pub fn add_attr(&mut self, attribute: &str) {
        &self.attributes.push(attribute.to_string());
    }
    
    pub fn add_child(&mut self, graphic: &'a Graphic) {
        &self.children.push(graphic);
    }

    pub fn element(&self) -> &String {
        &self.element
    }

    pub fn attributes(&self) -> &Vec<String> {
        &self.attributes
    }

    pub fn attr_as_str(&self) -> String {
        let mut out = String::new();
        for attribute in &self.attributes {
            out.push_str(attribute);
            out.push(' ');
        }
        out
    }
}