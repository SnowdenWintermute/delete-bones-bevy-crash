use bevy_app::bevy_main;
mod bevy_app;

pub struct SharedState {
    pub name: String,
}

fn main() {
    bevy_main();
}
