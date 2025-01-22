use std::default::Default;
use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture_ex, load_texture, Rect, Texture2D, Vec2};
use macroquad::texture::DrawTextureParams;
use crate::constants::TILE_SIZE;
use crate::core::entities::base_unit::SpritePath;
use crate::core::entities::team::Team;
use crate::core::traits::Drawable;

// TODO: world draw queue and UI draw queue

pub struct Entity {
    pub texture2d: Texture2D,
    pub player_rect: Rect,
    pub size: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub team: Team
}

impl Entity {
    async fn new(texture: SpritePath, position: Vec2, size: Vec2,
                 velocity: Vec2, rotation: f32, team: Team) -> Entity {
        let texture2d = load_texture(&texture.to_string()).await.unwrap();
        
        Entity {
            texture2d,
            player_rect: Rect::new(position.x, position.y, size.x, size.y),
            size,
            velocity,
            rotation,
            team,
        }
    }
}

impl Drawable for Entity {
    fn draw(&self) {
        draw_texture_ex(
            &self.texture2d,
            self.player_rect.x * TILE_SIZE as f32,
            self.player_rect.y * TILE_SIZE as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.player_rect.size() * TILE_SIZE as f32),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            }
        )
    }
}

pub struct GroundUnit {
    pub entity: Entity,
    pub body: Texture2D,
}

impl GroundUnit {
    /// Defaults a dagger at 0, 0 with size 1, 1 on team Serpulo.
    pub async fn default() -> Self {
        // TODO: ADD PROPER SPRITE!!!
        let entity = Entity::new(
            SpritePath::new("dagger.png".to_string()),
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            Default::default(),
            0.0,
            Team::Serpulo
        ).await;
        
        let body_path = SpritePath::new("dagger-base.png".to_string());
        
        Self {
            entity,
            body: load_texture(&body_path.to_string()).await.unwrap(),
        }
    }
}

impl Drawable for GroundUnit {
    fn draw(&self) {
        draw_texture_ex(
            &self.body,
            self.entity.player_rect.x * TILE_SIZE as f32,
            self.entity.player_rect.y * TILE_SIZE as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.entity.player_rect.size() * TILE_SIZE as f32),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            }
        );
        
        self.entity.draw();
    }
}

/// Due to the nature of Rust, you need to provide a variable
#[macro_export]
macro_rules! ground_unit {
    () => {
        Box::leak(Box::from(GroundUnit::default().await))
    }
}
