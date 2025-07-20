use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum States {
    MainMenu,
    Paused,
}

impl Default for States {
    fn default() -> Self {
        States::MainMenu
    }
}

mod object;
mod object_functions;

fn main() 
{

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_state::<States>();
    app.add_systems(Update, object::draw_static_objects);
    app.add_systems(Update, object_functions::handle_buttons);

    app.run();
}

