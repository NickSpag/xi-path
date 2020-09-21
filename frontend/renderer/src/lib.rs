pub mod pathfinder_renderer;

pub trait Renders : 
{
    fn render(&mut self);
}