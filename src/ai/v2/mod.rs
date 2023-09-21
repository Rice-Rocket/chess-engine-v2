use bevy::prelude::*;
use crate::state::AppState;

pub mod search;
pub mod evaluation;


fn load(
    mut commands: Commands,
) {
    commands.insert_resource(search::searcher::Searcher::default());
}

pub struct AIPluginV2;

impl Plugin for AIPluginV2 {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::LoadAI), load);
    }
}