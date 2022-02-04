use crate::State;

// This is a simple one, just the things that a state can return from update
pub enum UpdateStatus {
	Ok,
	ChangeState(Box<dyn State>),
}