use bevy::app::PluginGroupBuilder;
use bevy::math::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_experiments::physics::{Gravity, Position, Velocity};
use bevy_experiments::physics::systems::PhysicsPlugin;
use bevy_experiments::physics::PhysicsWorld;
use crate::physics::Spring;
use crate::utils::mesh_builder::MeshBuilder;
use super::AppState;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct CleanupFlag;

const STATE : AppState = AppState::PhysicsNew;

struct PhysicsNewPlugin;
impl Plugin for PhysicsNewPlugin {
    fn build(&self, app: &mut App) {
        let is_in_state = in_state(STATE);
        app
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, menu_system.run_if(is_in_state.clone()))
            .add_systems(OnExit(STATE), cleanup);
    }
}

pub struct PhysicsNewPluginGroup;

impl PluginGroup for PhysicsNewPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PhysicsNewPlugin)
            .add(PhysicsPlugin)
    }
}

pub fn init(
    app: &mut App
) {
    app.insert_resource(PhysicsWorld::default());
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    //  lights
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 2.0, 1.0),
        ..default()
    }).insert(CleanupFlag);

    //  camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
            },
        PanOrbitCamera::default()
    )).insert(CleanupFlag);

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Cuboid { half_size: Vec3::splat(0.5) })),
        material: materials.add(Color::YELLOW),
        transform: Transform::from_xyz(2., 1., 0.),
        ..Default::default()
    }).insert(CleanupFlag);

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(primitives::Plane3d { normal: Direction3d::from_xyz(0., 1., 0.).unwrap() })),
        material: materials.add(Color::VIOLET),
        transform: Transform::from_xyz(2., -1., 0.),
        ..Default::default()
    }).insert(CleanupFlag);

    //  Bounding World
    let physics_world = PhysicsWorld::default();
    commands.insert_resource(physics_world);
    let mut mesh_builder = MeshBuilder::create_with_uv_coordinates();
    mesh_builder.add_cube_inverted(Vec3::ZERO, Color::WHITE);

    let mesh_handle = mesh_builder.to_mesh(meshes);
    // Load the texture
    let texture_handle: Handle<Image> = asset_server.load("textures/debug_grid.png");
    let material_handle = materials.add( StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(texture_handle),
        ..default()
    });

    commands
        .spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                //  should also include translation to center.
                transform: Transform::from_xyz(
                    physics_world.bounds.center.x,
                    physics_world.bounds.center.y,
                    physics_world.bounds.center.z,
                ).with_scale(
                    Vec3::new(
                        physics_world.bounds.half_extents.x * 2.,
                        physics_world.bounds.half_extents.y * 2.,
                        physics_world.bounds.half_extents.z * 2.,
                    )
                ),
                ..Default::default()
            },
            CleanupFlag,
        ));
}

fn menu_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let context = contexts.ctx_mut();

    egui::Window::new("Material").show(context, |ui| {
        if ui.button("Shoot Ball").clicked() {
            // sphere
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(primitives::Sphere { radius: 0.5 })),
                    material: materials.add(Color::RED),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..Default::default()
                },
                Position(Vec3::new(0.0, 0.0, 0.0)),
                Velocity(Vec3::new(-3.0, 0.0, -20.0)),
                Gravity,
                CleanupFlag,
            ));
        }
        if ui.button("Shoot Pair").clicked() {
            // sphere A
            let particle_a = commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(primitives::Sphere { radius: 0.5 })),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..Default::default()
                },
                Position(Vec3::new(0.0, 0.0, 0.0)),
                Velocity(Vec3::new(-3.0, 0.0, -20.0)),
                Gravity,
                CleanupFlag,
            )).id();
            let particle_b = commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(primitives::Sphere { radius: 0.5 })),
                    material: materials.add(Color::GREEN),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..Default::default()
                },
                Position(Vec3::new(2.0, 1.0, 1.0)),
                Velocity(Vec3::new(-3.0, 0.0, -25.0)),
                Gravity,
                CleanupFlag,
            )).id();
            //     add a spring between the two
            commands.spawn((
                Spring {
                    particle_a,
                    particle_b,
                    rest_length: 2.0,
                    stiffness: 1.0,
                    damping: 1.0,
                },
                CleanupFlag,
            ));
        }
    });
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<CleanupFlag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
