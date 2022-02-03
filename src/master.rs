use crate::Context;
use crate::EmptyState;
use crate::State;

// This is the main container for the current state, and context. Context holds current resources (resources usually don't change btw), and the viewport
// And the viewport is just empty until the update and render
pub struct Master {
	pub state: Box<dyn State>,
	pub context: Context,
}

// Use this for quickly prototyping.
// It just makes everything empty, and the program will still run
impl Default for Master {
	fn default() -> Self {
		Self {
			state: Box::new(EmptyState {}),
			context: Context::default(),
		}
	}
}