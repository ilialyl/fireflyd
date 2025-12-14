use std::path::Path;

use mpris_server::PlaybackStatus;

use crate::player::Player;

#[test]
fn new() {
    assert!(Player::new().is_ok())
}

#[test]
fn stopped() {
    let player = Player::new().unwrap();
    assert_eq!(player.playback_status(), PlaybackStatus::Stopped);
}

#[test]
fn play() {
    let player = Player::new().unwrap();
    player
        .append(Path::new("../test_assets/test.flac"))
        .unwrap();
    player.play();
    assert_eq!(player.playback_status(), PlaybackStatus::Playing);
}

#[test]
fn pause() {
    let player = Player::new().unwrap();
    player
        .append(Path::new("../test_assets/test.flac"))
        .unwrap();
    player.pause();
    assert_eq!(player.playback_status(), PlaybackStatus::Paused);
}
