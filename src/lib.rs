// Kayak is an easy to use "Game Engine" for Rust!
// Licensed under the MIT license
// See https://github.com/eboatwright/kayak for more information!

// I just go ahead and import everything, so that you can just say: use kayak::*; instead of a prelude
pub mod context;
pub mod master;
pub mod resources;
pub mod state;
pub mod viewport;

pub use crate::context::*;
pub use crate::master::*;
pub use crate::resources::*;
pub use crate::state::*;
pub use crate::viewport::*;

use macroquad::prelude::*;

// Call this so kick off the game loop!
pub async fn start(master: &mut Master) {
    // Initialize the current state
    master.state.initialize(&mut master.context);

    // This is for screen scaling so that when the window size changes, the game view will scale appropriately
    let game_render_target = render_target(master.context.viewport.screen_size().x as u32, master.context.viewport.screen_size().y as u32);
    game_render_target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D {
        // I don't know why this is like this, but it's just Macroquad
        zoom: vec2(1.0 / master.context.viewport.screen_size().x * 2.0, 1.0 / master.context.viewport.screen_size().y * 2.0),
        render_target: Some(game_render_target),
        ..Default::default()
    };

    loop {
        // Update current scene
        master.state.update(&mut master.context);

        // This sets the render camera's position to the viewport position
        camera.target = master.context.viewport.position;

        set_camera(&camera);

        // The main clear background
        clear_background(DARKGRAY);

        // Render the current state
        master.state.render(&master.context);

        // Go back to screen space
        set_default_camera();

        // Calculate differences and scales between the scaled window size, and the game view size
        let game_diff = vec2(
            screen_width() / master.context.viewport.screen_size().x,
            screen_height() / master.context.viewport.screen_size().y,
        );
        let aspect_diff = game_diff.x.min(game_diff.y);
        // I store this, so that I can use it to get the mouse position later
        master.context.viewport.zoom = aspect_diff;

        let scaled_game_size = vec2(
            master.context.viewport.screen_size().x * aspect_diff,
            master.context.viewport.screen_size().y * aspect_diff,
        );

        let padding = vec2(
            (screen_width() - scaled_game_size.x) * 0.5,
            (screen_height() - scaled_game_size.y) * 0.5,
        );

        // This clears the screen space so that there will be black bars
        clear_background(BLACK);

        // Render the game view onto screen space
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