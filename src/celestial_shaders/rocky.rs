use super::{color_mix, light_direction};
use crate::fragment::Fragment;
use crate::noise::{fbm, smoothstep};
use raylib::prelude::Vector3;

const CLOUD_COVERAGE: f32 = 0.56;
const CLOUD_SPEED: f32 = 0.035;
const BUMP_STRENGTH: f32 = 0.34;
const PARALLAX_DEPTH: f32 = 0.075;
const CLOUD_SHADOW_OFFSET: f32 = 0.085;

fn terrain_height(point: Vector3) -> f32 {
    let continents = fbm(point * 3.1, 5);
    let mountains = smoothstep(0.58, 0.82, fbm(point * 13.0, 3));
    continents * 0.72 + mountains * 0.28
}

fn cloud_mask(point: Vector3, time: f32) -> f32 {
    let flow = Vector3::new(time * CLOUD_SPEED, 0.0, -time * CLOUD_SPEED * 0.57);
    smoothstep(
        CLOUD_COVERAGE,
        CLOUD_COVERAGE + 0.11,
        fbm(point * 6.0 + flow, 4),
    )
}

fn bumped_normal(point: Vector3, normal: Vector3) -> Vector3 {
    let reference = if normal.y.abs() < 0.9 {
        Vector3::new(0.0, 1.0, 0.0)
    } else {
        Vector3::new(1.0, 0.0, 0.0)
    };
    let tangent = normal.cross(reference).normalize();
    let bitangent = normal.cross(tangent).normalize();
    let epsilon = 0.018;
    let center = terrain_height(point);
    let dx = terrain_height((point + tangent * epsilon).normalize()) - center;
    let dy = terrain_height((point + bitangent * epsilon).normalize()) - center;
    (normal - tangent * (dx / epsilon) * BUMP_STRENGTH - bitangent * (dy / epsilon) * BUMP_STRENGTH)
        .normalize()
}

pub fn shade(fragment: &Fragment, time: f32, camera_pos: Vector3) -> Vector3 {
    let geometric_point = fragment.object_position.normalize();
    let view = (camera_pos - fragment.world_position).normalize();
    let tangent_view = view - fragment.normal * view.dot(fragment.normal);
    let initial_height = terrain_height(geometric_point);
    let point =
        (geometric_point - tangent_view * ((initial_height - 0.5) * PARALLAX_DEPTH)).normalize();

    let terrain = fbm(point * 3.1, 5);
    let detail = fbm(point * 17.0, 3);
    let land_mask = smoothstep(0.43, 0.49, terrain);
    let ocean = Vector3::new(0.012, 0.10, 0.31) * (0.72 + detail * 0.28);
    let vegetation = color_mix(
        Vector3::new(0.055, 0.22, 0.07),
        Vector3::new(0.38, 0.25, 0.075),
        smoothstep(0.50, 0.72, terrain),
    );
    let mountain = Vector3::new(0.55, 0.48, 0.40) * (0.7 + detail * 0.45);
    let land = color_mix(vegetation, mountain, smoothstep(0.66, 0.82, terrain));
    let mut surface = color_mix(ocean, land, land_mask);
    let ice = smoothstep(0.72, 0.92, point.y.abs() + fbm(point * 8.0, 2) * 0.16);
    surface = color_mix(surface, Vector3::new(0.82, 0.91, 0.96), ice);

    let normal = bumped_normal(point, fragment.normal);
    let light = light_direction();
    let diffuse = normal.dot(light).max(0.0);
    let night = smoothstep(0.12, -0.18, normal.dot(light));
    let half_vector = (light + view).normalize();
    let ocean_specular = normal.dot(half_vector).max(0.0).powf(52.0) * (1.0 - land_mask);
    let coast = 1.0 - smoothstep(0.0, 0.055, (terrain - 0.47).abs());
    let cities = smoothstep(0.68, 0.78, fbm(point * 38.0, 2)) * coast * land_mask * night;

    let clouds = cloud_mask(point, time) * 0.82;
    let shadow_point = (point - light * CLOUD_SHADOW_OFFSET).normalize();
    let cloud_shadow = cloud_mask(shadow_point, time) * diffuse * (1.0 - clouds) * 0.48;
    let illumination = (0.10 + diffuse * 0.90) * (1.0 - cloud_shadow);
    let mut color = surface * illumination;
    color += Vector3::new(0.38, 0.55, 0.80) * ocean_specular;
    color += Vector3::new(1.0, 0.48, 0.10) * cities * 1.45;
    color_mix(
        color,
        Vector3::new(0.92, 0.95, 1.0) * (0.32 + diffuse * 0.68),
        clouds,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_clouds_and_height_are_bounded() {
        let point = Vector3::new(0.3, 0.4, 0.866).normalize();
        assert!((0.0..=1.0).contains(&cloud_mask(point, 0.0)));
        assert_ne!(cloud_mask(point, 0.0), cloud_mask(point, 10.0));
        assert!((0.0..=1.0).contains(&terrain_height(point)));
    }
}
