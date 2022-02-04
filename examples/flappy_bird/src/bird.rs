use kayak::UpdateStatus;
use macroquad::audio::play_sound;
use macroquad::audio::PlaySoundParams;
use kayak::Context;
use crate::GameState;
use crate::Resources;
use macroquad::prelude::*;
use core::f32::consts::PI;

const BIRD_GRAVITY: f32 = 0.25;
const BIRD_FLAP_HEIGHT: f32 = -3.5;

pub struct Bird {
    pub transform: Rect,
    pub y_velocity: f32,
    pub rotation: f32,
    pub no_rotation: bool,
    pub started: bool,
    pub done_game_over_animation: bool,
    pub game_over_timer: f32,
}

impl Default for Bird {
    fn default() -> Self {
        Self {
            transform: Rect {
                x: -70.0,
                y: -31.0,
                w: 20.0,
                h: 19.0,
            },
            y_velocity: 0.0,
            rotation: 0.0,
            no_rotation: false,
            started: false,
            done_game_over_animation: false,
            game_over_timer: 10.0,
        }
    }
}

pub fn bird_update(game_state: &mut GameState, context: &mut Context) -> UpdateStatus {
    let resources: &Resources = context.get_resources();
    if game_state.bird.started
    && game_state.bird.y_velocity < BIRD_GRAVITY * 18.0 {
        game_state.bird.y_velocity += BIRD_GRAVITY;
        if game_state.bird.rotation < 90.0 {
            game_state.bird.rotation += 5.0;
        }
    }

    if (is_key_pressed(KeyCode::Space)
    || is_mouse_button_pressed(MouseButton::Left))
    && !game_state.game_over {
        game_state.bird.started = true;
        game_state.bird.y_velocity = BIRD_FLAP_HEIGHT;
        game_state.bird.rotation = -90.0;
        play_sound(resources.flap, PlaySoundParams {
            looped: false,
            volume: 1.0,
        });
    }

    if is_key_pressed(KeyCode::R) {
        game_state.bird.no_rotation = !game_state.bird.no_rotation;
    }

    game_state.bird.transform.y += game_state.bird.y_velocity * delta_time();

    if game_state.bird.transform.y < -100.0 {
        game_state.bird.y_velocity = 0.0;
    }

    game_state.bird.transform.y = clamp(
        game_state.bird.transform.y,
        -100.0,
        100.0 - game_state.bird.transform.h,
    );

    for pipe in game_state.pipes.iter_mut() {
        if Rect::overlaps(&game_state.bird.transform, &Rect {
            x: pipe.transform.x + 2.0,
            y: pipe.transform.y + pipe.transform.h * 0.5 + 26.0,
            w: pipe.transform.w - 4.0,
            h: pipe.transform.h * 0.5 - 26.0,
        }) {
            game_state.game_over = true;
        }

        if Rect::overlaps(&game_state.bird.transform, &Rect {
            x: pipe.transform.x + 2.0,
            y: pipe.transform.y,
            w: pipe.transform.w - 4.0,
            h: pipe.transform.h * 0.5 - 26.0,
        }) {
            game_state.game_over = true;
        }

        if !pipe.scored {
            if Rect::overlaps(&game_state.bird.transform, &Rect {
                x: pipe.transform.x + 10.0,
                y: pipe.transform.y + pipe.transform.h * 0.5 - 26.0,
                w: pipe.transform.w - 10.0,
                h: 52.0,
            }) {
                pipe.scored = true;
                game_state.score += 1;
                play_sound(resources.score, PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                });
            }
        }
    }

    if game_state.game_over {
        if game_state.bird.game_over_timer > 0.0 {
            game_state.bird.game_over_timer -= delta_time();
        }
        if !game_state.bird.done_game_over_animation {
            game_state.bird.done_game_over_animation = true;
            game_state.bird.y_velocity = BIRD_FLAP_HEIGHT;
            play_sound(resources.game_over, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
        }
        if (is_key_pressed(KeyCode::Space)
        || is_mouse_button_pressed(MouseButton::Left))
        && game_state.bird.game_over_timer <= 0.0 {
            return UpdateStatus::ChangeState(Box::new(GameState::default()));
        }
    }
    UpdateStatus::Ok
}

pub fn bird_render(game_state: &GameState, resources: &Resources) {
    draw_texture_ex(
        resources.bird,
        game_state.bird.transform.x.round(),
        game_state.bird.transform.y.round(),
        WHITE,
        DrawTextureParams {
            rotation: if game_state.bird.no_rotation {
                0.0
            } else {
                game_state.bird.rotation * PI / 180.0
            },
            ..Default::default()
        },
    );
}