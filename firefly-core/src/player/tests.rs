use mpris_server::PlaybackStatus;

use crate::player::Player;

#[test]
fn new() {
    assert!(Player::new().is_ok())
}

#[test]
fn play() {
    let player = Player::new().unwrap();
    assert_eq!(player.playback_status(), PlaybackStatus::Stopped)
}
