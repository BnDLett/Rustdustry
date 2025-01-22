// --------------------- WINDOW ---------------------
// Due to the nature of macros in Rust, if you change this, then you need to change the name in the
// main.rs file too.
pub const GAME_NAME: &str = "Rustdustry";

// --------------------- GAME ---------------------
// TODO: MAKE THIS NOT A CONSTANT!! BAD FOR CAMERA ZOOM!!!
pub const TILE_SIZE: i32 = 100;
/// The minimum size for all GenericDrawQueues. 
/// 
/// Reducing this will help reduce memory usage. 
/// 
/// However, if this capacity is exceeded, then it will take longer to push a new Drawable to the
/// draw queue. Typically, O(capacity) time.
pub const QUEUE_MINIMUM_SIZE: usize = 512;

// --------------------- PATHS ---------------------
pub const UNIT_SPRITE_PATH: &str = "sprites/";
pub const MUSIC_PATH: &str = "music/";
