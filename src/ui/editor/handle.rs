use bevy::{gltf::Gltf, prelude::*};

use super::Models;

#[derive(Resource, Clone)]
pub struct ModelHandles {
    handles: Vec<Handle<Scene>>,
}

pub fn get_model_handle(name: String, model_handles: ModelHandles) -> Handle<Scene> {
    for handle in model_handles.handles.iter() {
        if handle.clone().path().unwrap().to_string() == name {
            return handle.clone();
        }
    }
    return model_handles.handles[0].clone();
}

pub fn load_models_early(
    mut commands: Commands,
    asset_server_gltf: Res<Assets<Gltf>>,
    models: Res<Models>,
) {
    let mut handles: Vec<Handle<Scene>> = vec![];
    for asset in models.folder.iter() {
        if let Some(gltf) = asset_server_gltf.get(asset.clone()) {
            handles = [handles, gltf.scenes.clone()].concat();
        }
    }
    commands.insert_resource(ModelHandles { handles });
}
