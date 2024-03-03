use bevy::prelude::*;
use bevy::math::*;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use super::AppState;
use bevy_xpbd_3d::prelude::*;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct LocalStateFlag;

pub struct PhysicsBlocksPlugin;

const STATE : AppState = AppState::PhysicsBlocks;

impl Plugin for PhysicsBlocksPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Msaa::Off)// turn off multi sample anti aliasing
            //  should this be applied here to all plugins?
            .insert_resource(AmbientLight {
                brightness: 5.0,
                ..default()
            })
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, system.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 8.0),
        ..default()
    }).insert(LocalStateFlag);
    // Camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 6.5, 28.0).looking_at(Vec3::new(0., 8., 0.), Vec3::Y),
        ..default()
    })
        .insert(ScreenSpaceAmbientOcclusionBundle::default())   //  screen space ambient occlusion!
        .insert(LocalStateFlag);
    // Orion's preferred fighting arena: Infinite Flat Plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(8.0, 0.0, 8.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(primitives::Plane3d { normal: Direction3d::Y })),
            transform: Transform::from_scale(Vec3::splat(1000.)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
    )).insert(LocalStateFlag);

    let cube_mesh = meshes.add(Mesh::from(primitives::Cuboid { half_size: Vec3::splat(0.5)}));
    let material_red = materials.add(Color::rgb(0.8, 0.7, 0.6));
    let material_black = materials.add(Color::rgb(0.4, 0.2, 0.3));
    let material_bullet = materials.add(Color::WHITE);

    let size = 1.0;
    //  slightly smaller physics size to avoid overlapping initial position warning from physics solver.
    let smaller_size = size * 0.999;
    // Cube
    for x in -4 ..= 1 {
        for y in 0 ..= 8 {
            for z in -2 ..= 2 {
                commands.spawn((
                    RigidBody::Dynamic,
                    Mass(1.0),
                    AngularVelocity(Vec3::new(0.0, 0.0, 0.0)),
                    LinearVelocity(Vec3::new(0.0,0.0, 0.0)),
                    Collider::cuboid(smaller_size, smaller_size, smaller_size),
                    PbrBundle {
                        mesh: cube_mesh.clone(),
                        material:  if (x + y + z) % 2 == 0 { material_red.clone() } else { material_black.clone() },
                        transform: Transform::from_xyz(size + x as f32, size / 2. + y as f32, size + z as f32),
                        ..default()
                    },
                )).insert(LocalStateFlag);
            }
        }
    }
    //  bullet
    commands.spawn((
        RigidBody::Dynamic,
        Mass(20.0),
        AngularVelocity(Vec3::new(0.0, 0.0, 800.0)),
        LinearVelocity(Vec3::new(0.0,2.0, -80.0)),
        Collider::cuboid(size, size, size),
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: material_bullet.clone(),
            transform: Transform::from_xyz(0., 4.0, 40.0),
            ..default()
        },
    )).insert(LocalStateFlag);
}

fn system() {
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<LocalStateFlag>>) {
    println!("Physics Blocks cleanup");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

