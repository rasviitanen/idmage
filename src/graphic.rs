#[derive(Debug)]
pub struct Graphic {
    element: String,
    weight: f64,
    center: (f64, f64),
    attributes: Vec<String>,
    children: Vec<Graphic>
}

impl Graphic {
    pub fn new(element: &str) -> Graphic {
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

    pub fn set_focal_point(&mut self, x: f64, y: f64) {
        self.center = (x, y);
    }

    pub fn focal_point(&self) -> (f64, f64) {
        self.center
    }

    pub fn add_attr(&mut self, attribute: String) {
        &self.attributes.push(attribute);
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