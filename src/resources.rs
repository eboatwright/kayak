use std::any::Any;

pub trait ResourceContainer {
	fn as_any(&self) -> &dyn Any;
}

pub struct NoResources {}
impl ResourceContainer for NoResources {
	fn as_any(&self) -> &dyn Any { self as &dyn Any }
}

pub fn convert_resources<T: 'static + ResourceContainer>(resources: &dyn Any) -> &T {
	resources.downcast_ref::<T>().unwrap()
}