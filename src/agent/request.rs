use std::cmp::Ordering;
use canvas::Canvas;
use std::fmt;

pub type ImpactMetricValue = u8;

macro_rules! request {
    ($author:expr, $impact:expr, $modification:expr) => {{
        Request { 
            author: $author.to_owned(),
            impact: $impact,
            modification: Box::new($modification),
        }
    }};
}

pub struct Request {
    pub author: String,
    pub impact: ImpactMetricValue, // How big is my impact on the canvas, scale 0-100?
    pub modification: Box<Fn(&mut Canvas)>,
}

impl Request {
    pub fn execute(&self, canvas: &mut Canvas) {
        (self.modification)(canvas);
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request {{ {} @ {} }}", self.impact, self.author)
    }
}

impl Eq for Request {}

impl PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.impact == other.impact
    }
}

impl Ord for Request {
    fn cmp(&self, other: &Request) -> Ordering {
        self.impact.cmp(&other.impact)
    }
}

impl PartialOrd for Request {
    fn partial_cmp(&self, other: &Request) -> Option<Ordering> {
        Some(self.impact.cmp(&other.impact))
    }
}
