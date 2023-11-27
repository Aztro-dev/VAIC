use crate::ui::editor::part_selector;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct ModelHandles {
    names: Vec<String>,
    handles: Vec<Handle<Scene>>,
}

pub fn get_model_handle(name: String, model_handles: ModelHandles) -> Handle<Scene> {
    let mut index = 0;
    for (i, handle_name) in model_handles.names.iter().enumerate() {
        if name == *handle_name {
            index = i;
        }
    }
    return model_handles.handles[index].clone();
}

pub fn load_models_early(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles: Vec<Handle<Scene>> = vec![];
    let mut names: Vec<String> = vec![];
    for name in part_selector::get_parts().iter() {
        let file_path = part_selector::get_model_name(name);

        let formatted = format!("models/{file_path}#Scene0");

        handles.push(asset_server.load(formatted.clone()));
        names.push(formatted.clone());
    }
    commands.insert_resource(ModelHandles { names, handles });
}
