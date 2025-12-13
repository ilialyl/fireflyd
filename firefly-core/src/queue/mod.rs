use std::collections::VecDeque;

use crate::track::Track;

pub struct Queue {
    current_track: Track,
    previous_tracks: Vec<Track>,
    next_tracks: VecDeque<Track>,
}

