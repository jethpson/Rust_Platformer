use bevy::prelude::*;
use crate::States;

pub fn draw_static_objects(state: Res<State<States>>) {
    match state.get() {
        States::MainMenu => {
            // draw menu background
        },
        States::Paused => {
            // draw pause screen overlay
        },
    }
}