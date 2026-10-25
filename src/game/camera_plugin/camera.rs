use crate::ui::menus::AppState;
use crate::world::game_configs::CameraConfig;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::ops::sqrt;
use bevy::{camera::ScalingMode, prelude::*};

#[derive(Resource)]
pub struct CameraZoom {
    pub zoom: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraZoom { zoom: 1.0 })
            .add_systems(OnEnter(AppState::InGameRenderer), setup_camera)
            .add_systems(
                Update,
                (camera_movement, apply_zoom_smoothing).run_if(in_state(AppState::InGameRenderer)),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands, config: Res<CameraConfig>) {
    let look_at = Vec3::new(32.0 * 0.5, 0.0, 32.0 * 0.5);
    let dist = 32.0 * 0.85;
    let camera_pos = look_at + Vec3::new(dist, dist, dist);

    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config.camera_viewport_height,
            },
            near: -10_0000.0,
            far: 10_0000.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_pos).looking_at(look_at, Vec3::Y),
    ));

    commands.insert_resource(GlobalAmbientLight {
        // note можно потом сделать красивей
        color: Color::WHITE,
        brightness: 400.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadow_maps_enabled: config.shadow_maps_enabled,
            ..default()
        },
        Transform::from_xyz(-2.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_movement(
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut target_zoom: ResMut<CameraZoom>,
    config: Res<CameraConfig>,
) {
    let Ok((mut transform, projection)) = camera_q.single_mut() else {
        return;
    };

    let pan_speed = 20.0 * sqrt(target_zoom.zoom) * time.delta_secs() + config.camera_pan_speed;

    let iso_forward = Vec3::new(-1.0, 0.0, -1.0).normalize();
    let iso_right = Vec3::new(1.0, 0.0, -1.0).normalize();

    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation += iso_forward * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation -= iso_forward * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        transform.translation += iso_right * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        transform.translation -= iso_right * pan_speed;
    }

    let zoom_speed = 0.2 + config.camera_zoom_speed;
    if let Projection::Orthographic(_) = *projection {
        for mouse_wheel in mouse_wheel_reader.read() {
            let scroll = match mouse_wheel.unit {
                MouseScrollUnit::Line => mouse_wheel.y * zoom_speed,
                MouseScrollUnit::Pixel => mouse_wheel.y * zoom_speed * 0.05,
            };
            if config.invert_zoom {
                target_zoom.zoom = (target_zoom.zoom + scroll)
                    .clamp(config.min_orthographic_scale, config.max_orthographic_scale);
            } else {
                target_zoom.zoom = (target_zoom.zoom - scroll)
                    .clamp(config.min_orthographic_scale, config.max_orthographic_scale);
            }
        }
    }
}
fn apply_zoom_smoothing(
    mut camera_q: Query<&mut Projection, With<MainCamera>>,
    target_zoom: Res<CameraZoom>,
    time: Res<Time>,
) {
    let Ok(mut projection) = camera_q.single_mut() else {
        return;
    };

    let Projection::Orthographic(ref mut ortho) = *projection else {
        return;
    };

    let smoothness = 8.0;
    let t = 1.0 - (-smoothness * time.delta_secs()).exp();
    ortho.scale = ortho.scale.lerp(target_zoom.zoom, t);
}
