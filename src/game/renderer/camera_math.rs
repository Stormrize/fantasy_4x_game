use crate::world::game_config::GameConfig;
use bevy::camera::ScalingMode;
use bevy::prelude::{Transform, Window, IVec2, Vec3, Projection, OrthographicProjection};

pub fn camera_forward(transform: &Transform) -> Vec3 {
    transform.rotation * Vec3::NEG_Z
}

pub fn project_ray_to_ground(origin: Vec3, direction: Vec3) -> Option<Vec3> {
    if direction.y.abs() < f32::EPSILON {
        return None;
    }
    // origin + direction * t = 0
    // direction * t = -origin
    // t = -origin / direction
    let distance = -origin.y / direction.y;

    Some(origin + direction * distance)
}

pub fn camera_ground_point(transform: &Transform) -> Vec3 {
    project_ray_to_ground(transform.translation, camera_forward(transform))
        .unwrap_or(transform.translation)
}

pub fn chunk_coord_from_world_xz(world_x: f32, world_z: f32, config: &GameConfig) -> IVec2 {
    let chunk_size = config.chunk_size;
    let tile_size = config.tile_size;
    let chunk_world_size = chunk_size as f32 * tile_size;

    IVec2::new(
        (world_x / chunk_world_size).floor() as i32,
        (world_z / chunk_world_size).floor() as i32,
    )
}

pub fn orthographic_view_size(ortho: &OrthographicProjection, window: &Window) -> (f32, f32) {
    let aspect = (window.width() / window.height()).max(0.001);

    let (width, height) = match ortho.scaling_mode {
        ScalingMode::WindowSize => (window.width(), window.height()),
        ScalingMode::Fixed { width, height } => (width, height),
        ScalingMode::AutoMin {
            min_width,
            min_height,
        } => {
            if aspect > min_width / min_height {
                (min_height * aspect, min_height)
            } else {
                (min_width, min_width / aspect)
            }
        }
        ScalingMode::AutoMax {
            max_width,
            max_height,
        } => {
            if aspect < max_width / max_height {
                (max_height * aspect, max_height)
            } else {
                (max_width, max_width / aspect)
            }
        }
        ScalingMode::FixedVertical { viewport_height } => {
            (viewport_height * aspect, viewport_height)
        }
        ScalingMode::FixedHorizontal { viewport_width } => {
            (viewport_width, viewport_width / aspect)
        }
    };

    (width * ortho.scale, height * ortho.scale)
}

pub fn camera_chunk_bounds(
    transform: &Transform,
    projection: &Projection,
    window: &Window,
    config: &GameConfig,
) -> (IVec2, IVec2) {
    let camera_ground_point = camera_ground_point(transform);
    let chunk_coord =
        chunk_coord_from_world_xz(camera_ground_point.x, camera_ground_point.z, config);

    let mut min = chunk_coord;
    let mut max = chunk_coord;

    if let Projection::Orthographic(ortho) = projection {
        let (view_width, view_height) = orthographic_view_size(ortho, window);
        let right = transform.rotation * Vec3::X;
        let up = transform.rotation * Vec3::Y;
        let forward = camera_forward(transform);
        let origin = transform.translation;
        let half_right = right * (view_width * 0.5);
        let half_up = up * (view_height * 0.5);

        let corners = [
            origin - half_right - half_up,
            origin - half_right + half_up,
            origin + half_right - half_up,
            origin + half_right + half_up,
        ];

        for corner in corners {
            if let Some(point) = project_ray_to_ground(corner, forward) {
                let coord = chunk_coord_from_world_xz(point.x, point.z, config);
                min = min.min(coord);
                max = max.max(coord);
            }
        }
    }

    (min, max)
}

pub fn outside_bounds(coord: IVec2, min: IVec2, max: IVec2) -> bool {
    coord.x < min.x || coord.y < min.y || coord.x > max.x || coord.y > max.y
}
