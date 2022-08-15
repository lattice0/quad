extern crate bevy;
extern crate bevy_rapier3d;
use std::sync::atomic::AtomicBool;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(setup_graphics)
        .add_system(check_assets_ready)
        .init_resource::<AssetsLoading>()
        .run();
}

#[derive(Default)]
struct AssetsLoading(Vec<HandleUntyped>);

fn setup(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    let x_shape: Handle<Mesh> = server.load("quad.gltf#Mesh0/Primitive0");
    loading.0.push(x_shape.clone_untyped());
}

fn check_assets_ready(
    commands: Commands,
    server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;
    static SETUP_PHYSICS_CALLED: AtomicBool = AtomicBool::new(false);
    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            if !SETUP_PHYSICS_CALLED.load(std::sync::atomic::Ordering::Relaxed) {
                setup_physics(commands, server, meshes, materials);
                SETUP_PHYSICS_CALLED.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        camera: Camera {
            priority: 1,
            ..Default::default()
        },
        transform: Transform::from_xyz(-0.1, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Floor
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0));
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    //Drone
    let x_shape: Handle<Mesh> = asset_server.load("quad.gltf#Mesh0/Primitive0");

    let m = &meshes.get(&x_shape);
    let x_shape = Collider::from_bevy_mesh(m.unwrap(), &ComputedColliderShape::TriMesh).unwrap();
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(x_shape)
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
