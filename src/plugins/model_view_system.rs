use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::math::*;
use super::AppState;

//  This local state component is added to all entities we create in our system.
//  This makes it easy to query for and despawn all entities with this component on cleanup.
#[derive(Component)]
struct CleanupFlag;
const STATE : AppState = AppState::ModelViewSystem;

struct ModelViewSystemPlugin;
impl Plugin for ModelViewSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(STATE), setup)
            .add_systems(OnExit(STATE), cleanup)
            .add_systems(Update,
                 (
                    system,
                    frame_count_increment,
                    animate_model,
                    remove_component_for_testing,
                ).run_if(in_state(STATE))
            )
        ;
    }
}

pub struct ModelViewSystemPluginGroup;
impl PluginGroup for ModelViewSystemPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ModelViewSystemPlugin)
            .add(MyModelViewSystemPlugin { state: STATE })
    }
}
#[derive(Component, Debug, PartialEq)]
struct FrameCount(u32);

fn frame_count_increment(mut query: Query<&mut FrameCount>) {
    for mut frame_count in query.iter_mut() {
        frame_count.set_if_neq(FrameCount(frame_count.0 + 1));
    }
}

fn animate_model(time: Res<Time>, mut query: Query<&mut MyModel>)
{
    let speed = 2.0 as f32;
    for mut model in query.iter_mut() {
        model.0.y += speed * time.delta_seconds();
    }
}

fn remove_component_for_testing(
    mut commands: Commands,
    query: Query<(Entity, &FrameCount)>,
) {
    // After two seconds have passed the `Component` is removed.
    for (entity, frame_count) in query.iter() {
        if frame_count.0 >= 120 {
            commands.entity(entity).remove::<MyModel>();
        }
    }
}

////////////////////////////////////////////////////////////
//  BEGIN Model View System
////////////////////////////////////////////////////////////

#[derive(Component, Debug, PartialEq)]
struct MyModel(Vec3);

fn on_my_model_added(
    query: Query<(Entity, &MyModel), Without<Transform>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, my_model) in query.iter() {
        commands.entity(entity).insert((
            PbrBundle {
                mesh: meshes.add(Mesh::from(primitives::Cuboid { half_size: Vec3::splat(0.5)})),
                material:  materials.add(Color::rgb(0.9, 0.6, 0.4)),
                transform: Transform::from_xyz(1.0, 0.5, 1.0),
                ..default()
            },
            FrameCount(0),
        ));
        println!("on_my_model_added {:?} -> {:?}", entity, my_model);
    }
}
fn on_my_model_updated(
    mut query: Query<(&MyModel, &mut Transform), Changed<MyModel>>,
) {
    for (my_model, mut transform) in &mut query {
        transform.translation = my_model.0;
    }
}
fn on_my_model_removal(mut commands: Commands, mut removed: RemovedComponents<MyModel>) {
    for entity in removed.read() {
        println!("on_my_model_removal {:?}", entity);
        //  remove the entity
        commands.entity(entity).despawn_recursive();
    }
}

struct MyModelViewSystemPlugin {
    state: AppState
}
impl Plugin for crate::plugins::model_view_system::MyModelViewSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    on_my_model_added,
                    on_my_model_updated.after(on_my_model_added),
                ).run_if(in_state(self.state))
            )
            .add_systems(
                PostUpdate,
                on_my_model_removal.run_if(in_state(self.state))
            )
        ;
    }
}

////////////////////////////////////////////////////////////
//  END Model View System
////////////////////////////////////////////////////////////

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Light
    commands.spawn((
       PointLightBundle {
            point_light: PointLight {
                intensity: 1_000_000.,
                range: 100.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 8.0),
            ..default()
        },
       CleanupFlag,
    ));
    // Camera
    commands.spawn((
       Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-4.0, 6.5, 18.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        CleanupFlag
    ));
    // Infinite Flat Plane
    commands.spawn((
        CleanupFlag,
        PbrBundle {
            mesh: meshes.add(Mesh::from(primitives::Plane3d { normal: Direction3d::Y })),
            transform: Transform::from_scale(Vec3::splat(1000.)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));
    //  Model.
    commands.spawn((
        CleanupFlag,
        MyModel(Vec3::new(0.0, 1.0, 2.0)),
    ));
}

fn system() {
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<CleanupFlag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

