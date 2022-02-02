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

pub trait ResourceContainer {}

struct NoResources {}
impl ResourceContainer for NoResources {}

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

pub fn convert_resources<T: 'static + Clone + ResourceContainer>(resources: Box<dyn ResourceContainer>) -> T {
	((&resources as &dyn Any).downcast_ref::<T>().unwrap()).clone()
}

pub async fn start(master: &mut Master) {
	master.state.initialize();
	loop {
		master.state.update();

		master.state.render(&master.resources);

		next_frame().await
	}
}