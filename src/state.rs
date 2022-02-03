use crate::Viewport;
use crate::ResourceContainer;

pub trait State {
	fn initialize(&mut self);
	fn update(&mut self, viewport: &mut Viewport);
	fn render(&self, viewport: &Viewport, resources: &Box<dyn ResourceContainer>);
}

pub struct EmptyState {}
impl State for EmptyState {
	fn initialize(&mut self) {}
	fn update(&mut self, _viewport: &mut Viewport) {}
	fn render(&self, _viewport: &Viewport, _resources: &Box<(dyn ResourceContainer)>) {}
}