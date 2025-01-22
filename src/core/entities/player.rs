use crate::core::traits::Drawable;

struct Player<T: Drawable> {
    unit: T,
}

impl<T: Drawable> Player<T> {
    pub fn new(unit: T) -> Self {
        Self { unit }
    }
    
    pub fn move_unit(&mut self) {
        
    }
}
