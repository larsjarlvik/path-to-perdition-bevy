use bevy::{gltf::*, pbr::*, prelude::*, utils::HashMap};

#[derive(Resource)]
pub(crate) struct GltfModels(pub HashMap<String, Handle<Gltf>>);

/// Initializes a PbrBundle for a mesh using its name
pub(crate) fn get_bundles_from_model(
    model_name: &str,
    mesh_name: &str,
    assets: &Res<GltfModels>,
    gltfs: &Res<Assets<Gltf>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    transform: Option<Transform>,
) -> Option<Vec<PbrBundle>> {
    let model = assets.0.get(model_name).expect(&format!("Failed to find model {}!", model_name));

    if let Some(assets) = gltfs.get(&model) {
        let handle = &assets
            .named_meshes
            .get(mesh_name)
            .expect(&format!("No mesh named {} found!", mesh_name));

        let gltf_mesh = gltf_meshes.get(handle).expect("Handle found but could not get mesh!");

        Some(
            gltf_mesh
                .primitives
                .iter()
                .map(|primitive| PbrBundle {
                    mesh: primitive.mesh.clone(),
                    material: primitive.material.clone().unwrap_or_default(),
                    transform: transform.unwrap_or_default(),
                    ..Default::default()
                })
                .collect(),
        )
    } else {
        None
    }
}
