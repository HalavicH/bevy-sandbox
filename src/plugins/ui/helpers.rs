use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub trait UiBuilder {
    fn spawn_label(&mut self, text: &str, style: TextStyle) -> EntityCommands;
}

impl UiBuilder for ChildBuilder<'_> {
    fn spawn_label(&mut self, text: &str, style: TextStyle) -> EntityCommands {
        self.spawn((TextBundle::from_section(text, style), Label))
    }
}
