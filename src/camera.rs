// use bevy::prelude::*;
// use bevy::render::camera::ScalingMode;
// use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin, PanOrbitCameraSystemSet};

// pub struct CameraPlugin;

// impl Plugin for CameraPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(PanOrbitCameraPlugin)
//             .add_systems(Startup, setup)
//             .add_systems(Update, switch_projection.before(PanOrbitCameraSystemSet));
//     }
// }

// fn setup(mut commands: Commands) {
//     // help
//     commands.spawn(TextBundle {
//         text: Text {
//             sections: vec![TextSection {
//                 value: "Press R to switch projection".to_string(),
//                 ..Default::default()
//             }],
//             ..Default::default()
//         },
//         ..default()
//     });
//     // Camera
//     commands.spawn((
//         Camera3dBundle {
//             transform: Transform::from_translation(Vec3::new(5., 5., 5.)),
//             projection: Projection::Orthographic(OrthographicProjection {
//                 scaling_mode: ScalingMode::FixedVertical(1.0),
//                 ..default()
//             }),
//             ..default()
//         },
//         PanOrbitCamera {
//             zoom_upper_limit: Some(30.0),
//             zoom_lower_limit: Some(15.0),
//             ..default()
//         },
//     ));
// }

// fn switch_projection(
//     mut next_projection: Local<Projection>,
//     key_input: Res<ButtonInput<KeyCode>>,
//     mut camera_query: Query<(&mut PanOrbitCamera, &mut Projection)>,
// ) {
//     if key_input.just_pressed(KeyCode::KeyR) {
//         let Ok((mut camera, mut projection)) = camera_query.get_single_mut() else {
//             return;
//         };
//         std::mem::swap(&mut *next_projection, &mut *projection);
//         camera.force_update = true;
//     }
// }
