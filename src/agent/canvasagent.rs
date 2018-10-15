use canvas::Canvas;
use agent::request::Request;

/// A canvas agent is a small "expert", that excells at a certain task
/// For example, a canvas agent can be an expart at composition, or color
/// Canvas agents should be as tiny and narrow as possible
pub trait CanvasAgent {
    fn update(&mut self, canvas: &Canvas) -> Option<Request>;
    fn execute(&self, canvas: &mut Canvas);
}