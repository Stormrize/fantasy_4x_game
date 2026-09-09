//! Ортографическая камера для чанкового мира.

use crate::ui::menus::AppState;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::Affine2;
use bevy::math::ops::sqrt;
use bevy::mesh::PlaneMeshBuilder;
use bevy::render::render_resource::BindlessResourceType::Texture3d;
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

/// Маркер главной камеры — чтобы легко найти её в Query.
#[derive(Component)]
pub struct MainCamera;

/// Маркер фона
#[derive(Component)]
struct SpaceBackground;

// ─────────────────────────────────────────────────────────────────────────────

fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Загружаем текстуру космоса с повторением (как у вас)
    let texture_handle = asset_server.load_with_settings(
        "space_background_4.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            });
        },
    );

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        unlit: true, // Игнорирует DirectionalLight, чтобы космос не затенялся

        // Масштабируем UV: увеличиваем координаты в 100 раз.
        // Так как текстура в режиме Repeat, она повторится 100х100 раз по всей плоскости.
        uv_transform: Affine2::from_scale(Vec2::splat(100.0)),
        ..default()
    });

    // 3. Создаем стандартную плоскую 3D плоскость в Bevy 0.18
    let mesh_handle = meshes.add(Plane3d::default().mesh().size(200.0, 200.0));

    // 4. Спавним фон как 3D объект
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        // Опускаем ниже земли (например, на Y = -5.0), чтобы карта была поверх космоса
        Transform::from_xyz(0.0, -5.0, 0.0),
        SpaceBackground,
    ));
}

fn move_background(
    camera_query: Query<&Transform, (With<MainCamera>, Without<SpaceBackground>)>,
    mut bg_query: Query<&mut Transform, With<SpaceBackground>>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        if let Ok(mut bg_transform) = bg_query.single_mut() {
            // Привязываем X и Z к камере. Y остается фиксированным внизу
            bg_transform.translation.x = camera_transform.translation.x;
            bg_transform.translation.z = camera_transform.translation.z;
        }
    }
}

//TODO делать через конфиг
fn setup_camera(mut commands: Commands) {
    let look_at = Vec3::new(32.0 * 0.5, 0.0, 32.0 * 0.5);
    let dist = 32.0 * 0.85;
    let camera_pos = look_at + Vec3::new(dist, dist, dist);

    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 32.0 * 1.5,
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
            //note к настройкам
            shadow_maps_enabled: false,
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
) {
    let Ok((mut transform, mut projection)) = camera_q.single_mut() else {
        return;
    };

    let pan_speed = 20.0 * sqrt(target_zoom.zoom) * time.delta_secs();

    let iso_forward = Vec3::new(-1.0, 0.0, -1.0).normalize();
    let iso_right = Vec3::new(1.0, 0.0, -1.0).normalize();

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation += iso_forward * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation -= iso_forward * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation += iso_right * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation -= iso_right * pan_speed;
    }

    let zoom_speed = 0.2;
    if let Projection::Orthographic(ref _ortho) = *projection {
        for mouse_wheel in mouse_wheel_reader.read() {
            let scroll = match mouse_wheel.unit {
                MouseScrollUnit::Line => mouse_wheel.y * zoom_speed,
                MouseScrollUnit::Pixel => mouse_wheel.y * zoom_speed * 0.05,
            };

            target_zoom.zoom = (target_zoom.zoom - scroll).clamp(0.1, 100.0);
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
