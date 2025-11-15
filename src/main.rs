use bevy::prelude::*;

const MAP_SIZE: u32 = 16;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

// Plugin which controls game flow logic.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, create_map));
        app.add_systems(Update, handle_inputs);
    }
}

#[derive(Component)]
pub struct GameTile;

// TODO - process player input events to move block and camera around
fn handle_inputs() {}

fn create_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map_tile_mesh = meshes.add(Rectangle::new(1., 1.));
    let map_material = materials.add(Color::srgb_u8(152, 224, 112));

    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            // print game tiles
            commands.spawn((
                GameTile,
                Mesh3d(map_tile_mesh.clone()),
                MeshMaterial3d(map_material.clone()),
                Transform::from_xyz(x as f32, y as f32, 0.),
            ));
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube - i.e. player
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.5),
    ));

    // the sun
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            10_f32.to_radians(),
            10_f32.to_radians(),
            0.0,
        )),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -3.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
