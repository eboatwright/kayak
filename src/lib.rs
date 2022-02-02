use macroquad::prelude::*;
use std::any::Any;

pub trait State {
	fn initialize(&mut self);
	fn update(&mut self);
	fn render(&self, resources: &Box<dyn ResourceContainer>);
}

struct EmptyState {}
impl State for EmptyState {
	fn initialize(&mut self) {}
	fn update(&mut self) {}
	fn render(&self, _resources: &Box<(dyn ResourceContainer)>) {}
}

pub trait ResourceContainer {
	fn as_any(&self) -> &dyn Any;
}

struct NoResources {}
impl ResourceContainer for NoResources {
	fn as_any(&self) -> &dyn Any { self as &dyn Any }
}

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

pub fn convert_resources<T: 'static + ResourceContainer>(resources: &dyn Any) -> &T {
	resources.downcast_ref::<T>().unwrap()
}

pub async fn start(master: &mut Master) {
	master.state.initialize();
	loop {
		master.state.update();

		master.state.render(&master.resources);

		next_frame().await
	}
}