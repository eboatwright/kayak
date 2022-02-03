use crate::NoResources;
use crate::ResourceContainer;
use crate::Viewport;

// This is just to simplify state functions so they just take the state and context
pub struct Context {
	pub resources: Box<dyn ResourceContainer>,
	pub viewport: Viewport,
}

impl Default for Context {
	fn default() -> Self {
		Self {
			resources: Box::new(NoResources {}),
			viewport: Viewport::default(),
		}
	}
}