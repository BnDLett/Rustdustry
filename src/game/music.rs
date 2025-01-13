use macroquad::audio;
use macroquad::audio::*;
use crate::constants::MUSIC_PATH;

/// Loads all music related content. <br>
/// At a "proof of concept" point.
/// 
/// TODO: IMPROVE THIS!!
pub async fn load() {
    let game1_path = MUSIC_PATH.to_owned() + "game1.ogg";
    let game1 = load_sound(&game1_path).await.unwrap();
    
    play_sound(&game1, PlaySoundParams { looped: true, volume: 1.0 });
}
