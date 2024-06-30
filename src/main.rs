use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use bevy_ghx_proc_gen::{
    bevy_ghx_grid::ghx_grid::coordinate_system::{Cartesian2D, Cartesian3D},
    gen::{default_bundles::PbrMesh, simple_plugin::ProcGenSimplePlugin},
};
use bevy_ghx_utils::camera::update_pan_orbit_camera;
use ghx::{
    pillars::{setup_generator, setup_scene, ASSETS_SCALE, GENERATION_VIEW_MODE},
    plugin::ProcGenExamplesPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Procedural Generation Game".into(),
                    ..default()
                }),
                ..default()
            }),
            ProcGenExamplesPlugin::<Cartesian3D, Handle<Scene>>::new(
                GENERATION_VIEW_MODE,
                ASSETS_SCALE,
            ),
        ))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(ProcGenSimplePlugin::<Cartesian2D, PbrMesh>::new())
        .add_systems(Startup, (setup_scene, setup_generator))
        .add_systems(Update, update_pan_orbit_camera)
        .run();
}
