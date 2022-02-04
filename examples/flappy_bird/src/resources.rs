use macroquad::audio::Sound;
use macroquad::prelude::Texture2D;
use kayak::ResourceContainer;
use std::any::Any;

pub struct Resources {
    pub background: Texture2D,
    pub pipes: Texture2D,
    pub bird: Texture2D,

    pub flap: Sound,
    pub game_over: Sound,
    pub score: Sound,
}

impl ResourceContainer for Resources {
    fn as_any(&self) -> &dyn Any { self as &dyn Any }
}