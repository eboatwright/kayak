use crate::*;

pub struct Master {
	pub state: Box<dyn State>,
	pub resources: Box<dyn ResourceContainer>,
}

impl Default for Master {
	fn default() -> Self {
		Self {
			state: Box::new(EmptyState {}),
			resources: Box::new(NoResources {}),
		}
	}
}