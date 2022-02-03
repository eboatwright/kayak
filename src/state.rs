use crate::Viewport;
use crate::ResourceContainer;

// Implement this trait into a struct that has all your state data into it
pub trait State {
	fn initialize(&mut self, viewport: &mut Viewport);
	fn update(&mut self, viewport: &mut Viewport);
	fn render(&self, viewport: &Viewport, resources: &Box<dyn ResourceContainer>);
}

// An empty state for quickly prototyping (See Master::default())
pub struct EmptyState {}
impl State for EmptyState {
	fn initialize(&mut self, _viewport: &mut Viewport) {}
	fn update(&mut self, _viewport: &mut Viewport) {}
	fn render(&self, _viewport: &Viewport, _resources: &Box<(dyn ResourceContainer)>) {}
}