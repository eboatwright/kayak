// Kayak is an easy to use "Game Engine" for Rust!
// Licensed under the MIT license
// See https://github.com/eboatwright/kayak for more information!

pub mod master;
pub mod resources;
pub mod state;
pub mod viewport;

pub use crate::master::*;
pub use crate::resources::*;
pub use crate::state::*;
pub use crate::viewport::*;

use macroquad::prelude::*;

pub async fn start(master: &mut Master, mut viewport: Viewport) {
    master.state.initialize();

    let game_render_target = render_target(viewport.screen_size().x as u32, viewport.screen_size().y as u32);
    game_render_target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D {
        zoom: vec2(1.0 / viewport.screen_size().x * 2.0, 1.0 / viewport.screen_size().y * 2.0),
        render_target: Some(game_render_target),
        ..Default::default()
    };

    loop {
        master.state.update(&mut viewport);

        camera.target = viewport.position;

        set_camera(&camera);

        clear_background(DARKGRAY);

        master.state.render(&viewport, &master.resources);

        set_default_camera();

        let game_diff = vec2(
            screen_width() / viewport.screen_size().x,
            screen_height() / viewport.screen_size().y,
        );
        let aspect_diff = game_diff.x.min(game_diff.y);
        viewport.zoom = aspect_diff;

        let scaled_game_size = vec2(
            viewport.screen_size().x * aspect_diff,
            viewport.screen_size().y * aspect_diff,
        );

        let padding = vec2(
            (screen_width() - scaled_game_size.x) * 0.5,
            (screen_height() - scaled_game_size.y) * 0.5,
        );

        clear_background(BLACK);

        draw_texture_ex(
            game_render_target.texture,
            padding.x.round(),
            padding.y.round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(scaled_game_size.floor()),
                ..Default::default()
            },
        );

        next_frame().await
    }
}