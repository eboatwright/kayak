use crate::EmptyState;
use crate::State;
use crate::ResourceContainer;
use crate::NoResources;

// This is the main container for the current state, and current resources (resources usually don't change btw)
pub struct Master {
	pub state: Box<dyn State>,
	pub resources: Box<dyn ResourceContainer>,
}

// Use this for quickly prototyping.
// It just makes everything empty, and the program will still run
impl Default for Master {
	fn default() -> Self {
		Self {
			state: Box::new(EmptyState {}),
			resources: Box::new(NoResources {}),
		}
	}
}