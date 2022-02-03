use std::any::Any;

// Implement this trait into a struct with all your resources into it
pub trait ResourceContainer {
	fn as_any(&self) -> &dyn Any;
}

// An empty Resources type for quickly prototyping. (See Master::default())
pub(crate) struct NoResources {}
impl ResourceContainer for NoResources {
	fn as_any(&self) -> &dyn Any { self as &dyn Any }
}

// This converts an &dyn Any into a usable Resources struct
pub fn convert_resources<T: 'static + ResourceContainer>(resources: &dyn Any) -> &T {
	resources.downcast_ref::<T>().unwrap()
}