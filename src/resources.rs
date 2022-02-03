use std::any::Any;

// Implement this trait into a struct with all your resources into it
pub trait ResourceContainer {
	// This is required to have custom Resource structs :(
	fn as_any(&self) -> &dyn Any;
}

// Basically just do this
// pub struct MyResourceStruct {
// 	Insert resources here
// }
// impl ResourceContainer for MyResourceStruct {
// 	fn as_any(&self) -> &dyn Any {
// 		self as &dyn Any
// 	}
// }
// I would make this a macro, but I don't know how :(

// An empty Resources type for quickly prototyping. (See Master::default())
pub(crate) struct NoResources {}
impl ResourceContainer for NoResources {
	fn as_any(&self) -> &dyn Any { self as &dyn Any }
}