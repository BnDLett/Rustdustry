use std::fmt;
use std::fmt::Display;
use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;
use crate::constants::UNIT_SPRITE_PATH;
use crate::game::units::team::Team;

/// a VERY base structure for units. Anuke's way of handling units is the fourth circle of hell, so
/// I am not doing a 1:1 copy.
pub struct BaseUnit {
    pub sprite: Texture2D,
    pub speed: f32,
    pub position: Vec2,
    pub rotation: f32,
    pub target_rotation: f32,
    pub team: Team,
    
    pub boost_multiplier: f32,
    pub rotation_speed: f32,
    pub acceleration: f32,
    pub hit_size: f32,
    
    pub health: f32,
    pub armor: f32,
    pub build_range: f32,
}

pub struct SpritePath {
    path: String
}

impl SpritePath {
    pub fn new(file_name: String) -> SpritePath {
        let sprite_path = UNIT_SPRITE_PATH.to_owned() + file_name.as_str();
        SpritePath { path: sprite_path }
    }
}

impl Display for SpritePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.path)
    }
}
