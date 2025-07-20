use bevy::prelude::*;
use crate::States;

pub fn handle_buttons(
    state: Res<State<States>>,
    mut next_state: ResMut<NextState<States>>,
    // Add Querys/Inputs as needed
) {
    match state.get() {
        States::MainMenu => {
            // handle menu button logic
            // next_state.set(States::InGame); to transition
        },
        States::Paused => {
            // handle pause screen buttons
        },
    }
}