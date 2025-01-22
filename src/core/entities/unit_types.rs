use macroquad::color::*;
use macroquad::prelude::{draw_texture, draw_texture_ex, load_texture, screen_height, screen_width, DrawTextureParams, Texture2D, Vec2};
use crate::constants::TILE_SIZE;
use crate::core::entities::base_unit::{BaseUnit, SpritePath};
use crate::core::entities::team::Team;

pub struct Flare {
    pub base_unit: BaseUnit
}

// waoh! composition!
/// The flare unit.
impl Flare {
    pub async fn new() -> Flare {
        let sprite_path = SpritePath::new("flare.png".to_string());
        let texture: Texture2D = load_texture(&sprite_path.to_string()).await.unwrap();
        
        Flare {
            base_unit: BaseUnit { 
                sprite: texture,
                max_speed: 2.7,
                position: Default::default(),
                velocity: Default::default(),
                drag: 0.04,
                rotation: 0.0,
                target_rotation: 0.0,
                team: Team::Serpulo,
                boost_multiplier: 0.0,
                rotation_speed: 10.0,
                acceleration: 0.08,
                hit_size: 9.0,
                health: 70.0,
                armor: 0.0,
                build_range: 0.0,
            }
        }
    }
    
    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.base_unit.sprite, 
            self.base_unit.position.x * TILE_SIZE as f32,
            self.base_unit.position.y * TILE_SIZE as f32, 
            WHITE, 
            DrawTextureParams { 
                dest_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)), 
                source: None, 
                rotation: self.base_unit.target_rotation, 
                flip_x: false, 
                flip_y: false, 
                pivot: None, 
            }
        );
    }
}
