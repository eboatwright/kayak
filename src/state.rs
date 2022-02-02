use crate::ResourceContainer;

pub trait State {
	fn initialize(&mut self);
	fn update(&mut self);
	fn render(&self, resources: &Box<dyn ResourceContainer>);
}

pub struct EmptyState {}
impl State for EmptyState {
	fn initialize(&mut self) {}
	fn update(&mut self) {}
	fn render(&self, _resources: &Box<(dyn ResourceContainer)>) {}
}