use bevy::{math::*, pbr::*, prelude::*};

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    let character = ass.load("models/character.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: character,
        transform: Transform::from_xyz(0.0, -1.0, 0.0).with_scale(vec3(0.01, 0.01, 0.01)),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 10.0,
                subdivisions: 1,
            })),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 0.1,
                ..default()
            }),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 12000.0,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 3,
            maximum_distance: 50.0,
            ..default()
        }
        .into(),
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, -0.5)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
