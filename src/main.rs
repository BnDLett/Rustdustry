mod constants;
mod game;

use std::f32::consts::PI;
use macroquad::prelude::*;
use crate::constants::{GAME_NAME, TILE_SIZE};
use crate::game::music;
use self::game::units::unit_types::Flare;

#[macroquad::main("Rustdustry")]
async fn main() {
    set_pc_assets_folder("src/assets/");
    
    let mut player = Flare::new().await;
    let mut screen_center: Vec2;
    let mut position_change: f32;
    
    music::load().await;

    loop {
        position_change = get_frame_time() * (player.base_unit.speed * TILE_SIZE as f32);
        // player.base_unit.target_rotation = 0.0;

        // TODO: IMPROVE THIS!! PROOF OF CONCEPT!!!
        if is_key_down(KeyCode::W) {
            player.base_unit.position.y -= position_change;
            player.base_unit.target_rotation = 0.0;
        } 
        
        if is_key_down(KeyCode::S) {
            player.base_unit.position.y += position_change;
            player.base_unit.target_rotation = PI;
        } 
        
        if is_key_down(KeyCode::A) {
            player.base_unit.position.x -= position_change;
            player.base_unit.target_rotation = PI * 1.5;
        } 
        
        if is_key_down(KeyCode::D) {
            player.base_unit.position.x += position_change;
            player.base_unit.target_rotation = PI * 0.5;
        }

        screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        clear_background(BLACK);

        set_camera(&Camera2D {
            rotation: 0.0,
            zoom: vec2(screen_height() * 0.000005, screen_width() * 0.000005),
            target: player.base_unit.position + player.base_unit.sprite.size() / 2.,
            offset: Default::default(),
            render_target: None,
            viewport: None,
        });

        draw_text(GAME_NAME, screen_center.x - (GAME_NAME.len() as f32 * 5f32), 20.0, 30.0, LIGHTGRAY);
        player.draw();

        next_frame().await
    }
}
