use bevy::{math::vec3, pbr::*, prelude::*};

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_system(bevy::window::close_on_esc)
        .add_system(setup_scene_once_loaded)
        .add_startup_system(setup_scene);

    app.run();
}

/// set up a simple 3D scene
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Animations(vec![
        asset_server.load("models/character.glb#Animation4")
    ]));

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/character.glb#Scene0"),
        transform: Transform::from_xyz(0.0, -1.0, 0.0).with_scale(vec3(0.01, 0.01, 0.01)),
        ..default()
    });
}

fn setup_scene_once_loaded(animations: Res<Animations>, mut player: Query<&mut AnimationPlayer>) {
    if let Ok(mut player) = player.get_single_mut() {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}
