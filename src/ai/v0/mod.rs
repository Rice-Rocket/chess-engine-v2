use bevy::prelude::*;
use crate::state::AppState;

pub mod search;
pub mod evaluation;


fn load(
    mut commands: Commands,
) {
    commands.insert_resource(search::searcher::SearcherV0::default());
}

pub struct AIPluginV0;

impl Plugin for AIPluginV0 {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::LoadAI), load);
    }
}