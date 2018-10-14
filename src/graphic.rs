//! A single or grouped element that caa be placed on a canvas.
use math::helpers::Coordinate;

#[derive(Debug)]
pub struct Graphic {
    element: String,
    math_expr: Option<String>,
    weight: f64,
    pub center: Coordinate,
    attributes: Vec<String>,
    text: Option<String>,
    children: Vec<Graphic>
}

impl Graphic {
    pub fn new(element: &str) -> Graphic {
        Graphic {
            element: element.to_string(),
            math_expr: None,
            weight: 0.0,
            center: Coordinate{x: 0.0, y: 0.0, z: 0.0},
            text: None,
            attributes: Vec::new(),
            children: Vec::new()
        }
    }

    pub fn get_center(&self) -> &Coordinate {
        &self.center
    }

    pub fn add_weight(&mut self, weight: f64) {
        self.weight += weight;
    }

    pub fn weight(&self) -> (f64) {
        self.weight
    }

    pub fn text(&self) -> &Option<String> {
        &self.text
    }

    pub fn add_text(&mut self, text: &str) {
        self.text = Some(text.to_string());
    }

    pub fn add_attr(&mut self, attribute: String) {
        self.attributes.push(attribute);
    }
    
    pub fn add_child(&mut self, graphic: Graphic) {
        self.children.push(graphic);
    }

    pub fn children(&self) -> &Vec<Graphic> {
        &self.children
    }

    pub fn element(&self) -> &String {
        &self.element
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