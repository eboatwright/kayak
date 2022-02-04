use crate::Context;
use crate::UpdateStatus;

// Implement this trait into a struct that has all your state data into it
pub trait State {
	fn update(&mut self, context: &mut Context) -> UpdateStatus;
	fn render(&self, context: &Context);
}

// An empty state for quickly prototyping (See Master::default())
pub struct EmptyState {}
impl State for EmptyState {
	fn update(&mut self, _context: &mut Context) -> UpdateStatus { UpdateStatus::Ok }
	fn render(&self, _context: &Context) {}
}

impl Default for EmptyState {
	// The default function serves as an initialize function
	fn default() -> Self {
		Self {
		}
	}
}