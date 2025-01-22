use macroquad::camera::Camera2D;
use macroquad::math::Rect;
use macroquad::prelude::{next_frame, screen_height, screen_width, set_camera, Vec2};
use crate::constants::{QUEUE_MINIMUM_SIZE, TILE_SIZE};
use crate::core::entities::entity::GroundUnit;
use crate::core::traits::Drawable;
use crate::ground_unit;
use std::sync::{RwLock, RwLockReadGuard};
use macroquad::time::get_frame_time;

pub struct GenericDrawQueue<T: Drawable> {
    queue: Vec<T>,
}

impl<'a, T: Drawable + 'a> GenericDrawQueue<T> {
    pub fn new() -> Self {
        GenericDrawQueue { queue: Vec::with_capacity(QUEUE_MINIMUM_SIZE) }
    }
    
    pub fn add(&mut self, to_add: T) -> usize {
        let index = self.queue.len();
        self.queue.push(to_add);
        index
    }
    
    pub fn remove(&mut self, index: usize) {
        self.queue.remove(index);
    }
    
    pub fn get(&self, index: usize) -> &T {
        &self.queue[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.queue[index]
    }
    
    pub fn draw_all(&self) {
        self.queue.iter().for_each(|drawable| drawable.draw());
    }
}

pub async fn test() {
    let ground_unit_draw_queue = RwLock::new(GenericDrawQueue::new());
    
    let unit_index = ground_unit_draw_queue.write().unwrap().add(GroundUnit::default().await);
    let second_index = ground_unit_draw_queue.write().unwrap().add(GroundUnit::default().await);
    
    ground_unit_draw_queue.write().unwrap().get_mut(second_index).entity.player_rect.x += 1.0;
    ground_unit_draw_queue.write().unwrap().get_mut(second_index).entity.player_rect.y += 1.0;
    
    let mut player_camera: Camera2D;
    // The lock for the ground unit draw queue.
    let guq_read_lock = ground_unit_draw_queue.read().unwrap();
    let mut unit_position;
    let mut unit_size;
    
    loop {
        unit_position = guq_read_lock.get(unit_index).entity.player_rect.point();
        unit_size = guq_read_lock.get(unit_index).entity.player_rect.size();
        
        player_camera = Camera2D::from_display_rect(
            Rect {
                x: ((unit_position.x + (unit_size.x / 2.0)) * TILE_SIZE as f32) - (screen_width() / 2.0),
                y: ((unit_position.y + (unit_size.y / 2.0)) * TILE_SIZE as f32) + (screen_height() / 2.0),
                w: screen_width(),
                h: -screen_height(),
            }
        );
        
        set_camera(&player_camera);
        
        println!("{:?}", 1.0 / get_frame_time());

        guq_read_lock.draw_all();
        next_frame().await
    }
}
