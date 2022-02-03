use crate::Context;

// Implement this trait into a struct that has all your state data into it
pub trait State {
	fn initialize(&mut self, context: &mut Context);
	fn update(&mut self, context: &mut Context);
	fn render(&self, context: &Context);
}

// An empty state for quickly prototyping (See Master::default())
pub struct EmptyState {}
impl State for EmptyState {
	fn initialize(&mut self, _context: &mut Context) {}
	fn update(&mut self, _context: &mut Context) {}
	fn render(&self, _context: &Context) {}
}