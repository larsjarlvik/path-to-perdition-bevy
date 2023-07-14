use bevy::{gltf::*, math::vec3, pbr::*, prelude::*, utils::*};
mod components;
mod plugins;
mod resources;
mod systems;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(plugins::MipmapGeneratorPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 50.0))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(FixedUpdate, systems::movement)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, setup_scene_once_loaded)
        .add_systems(Update, plugins::generate_mipmaps::<StandardMaterial>);

    app.run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut resource_map: HashMap<String, Handle<Gltf>> = HashMap::new();
    resource_map.insert("character".into(), asset_server.load("models/character.glb"));
    resource_map.insert("skeleton".into(), asset_server.load("models/skeleton.glb"));

    commands.insert_resource(resources::GltfModels(resource_map));

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 6.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 12000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, -0.5)),
        ..default()
    });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Cube::new(1.0).into()),
    //     ..Default::default()
    // });

    commands.insert_resource(Animations(vec![asset_server.load("models/character.glb#Animation0")]));
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    assets: Res<resources::GltfModels>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        let character_bundles = resources::get_bundles_from_model(
            "character",
            "character",
            &assets,
            &gltfs,
            &gltf_meshes,
            Some(Transform::from_scale(vec3(0.01, 0.01, 0.01))),
        );

        if let Some(bundles) = character_bundles {
            commands.spawn_batch(bundles);
            *done = true;
        }

        // let skeleton_bundles = resources::get_bundles_from_model(
        //     "skeleton",
        //     "skeleton",
        //     &assets,
        //     &gltfs,
        //     &gltf_meshes,
        //     Some(Transform::from_translation(vec3(2.0, 0.0, 0.0))),
        // );

        // if let Some(bundles) = skeleton_bundles {
        //     commands.spawn_batch(bundles);
        //     *done = true;
        // }

        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
        }
    }
}
