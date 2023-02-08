use bevy::{prelude::*};

mod kiddyboids;

fn main() {
    kiddyboids::run(App::new());
}