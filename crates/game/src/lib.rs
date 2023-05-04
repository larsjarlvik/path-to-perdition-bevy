use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[bevy_main]
pub fn main() {
    App::new()
        .insert_resource(Msaa::Sample8)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Torus {
            radius: 2.0,
            ring_radius: 0.5,
            subdivisions_segments: 128,
            subdivisions_sides: 32,
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
