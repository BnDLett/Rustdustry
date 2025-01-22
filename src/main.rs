mod constants;
mod core;
mod game;

use std::arch::x86_64::_mm256_testnzc_si256;
use std::f32::consts::PI;
use macroquad::prelude::*;
use crate::core::music;
use self::core::entities::unit_types::Flare;
use crate::constants::{GAME_NAME, TILE_SIZE};
use crate::core::generic_draw_queue::test;

#[macroquad::main("Rustdustry")]
async fn main() {
    set_pc_assets_folder("src/assets/");
    
    let mut player = Flare::new().await;
    let mut screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    
    let mut velocity_change: f32;
    let mut player_speed: f32;
    let mut player_can_accelerate: f32;

    let mut player_camera: Camera2D;
    #[allow(unused_mut)]
    let mut zoom = 1f32;
    
    // music::load().await;
    
    test().await;

    // loop {
    //     player_camera = Camera2D::from_display_rect(
    //         Rect {
    //             x: (player.base_unit.position.x * TILE_SIZE as f32) - (screen_center.x / zoom) + (player.base_unit.sprite.size().x / 1.0),
    //             y: (player.base_unit.position.y * TILE_SIZE as f32) + (screen_center.y / zoom) + (player.base_unit.sprite.size().x / 1.0),
    //             w: screen_width() / zoom,
    //             h: -screen_height() / zoom,
    //         }
    //     );
    //     screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    //     
    //     // This is just a glorified way of determining the player's acceleration.
    //     player_speed = player.base_unit.velocity.length();
    //     player_can_accelerate = ((player.base_unit.max_speed) >= player_speed) as i32 as f32;
    //     velocity_change = player_can_accelerate * player.base_unit.acceleration;
    // 
    //     set_default_camera();
    //     
    //     // player.base_unit.target_rotation = 0.0;
    // 
    //     // TODO: IMPROVE THIS!! PROOF OF CONCEPT!!!
    //     if is_key_down(KeyCode::W) {
    //         player.base_unit.velocity.y -= velocity_change;
    //         player.base_unit.target_rotation = 0.0;
    //     }
    //     
    //     if is_key_down(KeyCode::S) {
    //         player.base_unit.velocity.y += velocity_change;
    //         player.base_unit.target_rotation = PI;
    //     } 
    //     
    //     if is_key_down(KeyCode::A) {
    //         player.base_unit.velocity.x -= velocity_change;
    //         player.base_unit.target_rotation = PI * 1.5;
    //     } 
    //     
    //     if is_key_down(KeyCode::D) {
    //         player.base_unit.velocity.x += velocity_change;
    //         player.base_unit.target_rotation = PI * 0.5;
    //     }
    // 
    //     player.base_unit.velocity -= player.base_unit.velocity * player.base_unit.drag;
    //     player.base_unit.position += player.base_unit.velocity * get_frame_time();
    // 
    //     println!("{}", 1.0 / get_frame_time());
    //     
    //     clear_background(BLACK);
    // 
    //     set_camera(&player_camera);
    // 
    //     draw_text(GAME_NAME, 0.0, 0.0, 30.0, LIGHTGRAY);
    //     player.draw();
    // 
    //     next_frame().await
    // }
}
