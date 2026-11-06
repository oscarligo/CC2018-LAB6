use super::{color_mix, light_direction};
use crate::fragment::Fragment;
use crate::noise::{fbm, smoothstep};
use raylib::prelude::Vector3;

pub fn shade(fragment: &Fragment, time: f32) -> Vector3 {
    let point = fragment.object_position.normalize();
    let storm_center = Vector3::new(0.58, -0.18, -0.78).normalize();
    let delta = point - storm_center;
    let distance = (delta.y * delta.y * 5.5 + delta.x * delta.x + delta.z * delta.z).sqrt();
    let influence = 1.0 - smoothstep(0.10, 0.46, distance);
    let angle = delta.z.atan2(delta.x);
    let twist = influence * (2.4 - distance * 3.0) + time * 0.05 * influence;
    let (sin_twist, cos_twist) = twist.sin_cos();
    let warped = Vector3::new(
        storm_center.x + delta.x * cos_twist - delta.z * sin_twist,
        point.y + angle.sin() * influence * 0.055,
        storm_center.z + delta.x * sin_twist + delta.z * cos_twist,
    )
    .normalize();

    let turbulence = fbm(warped * 4.5 + Vector3::new(time * 0.025, 0.0, 0.0), 4);
    let latitude = warped.y + (turbulence - 0.5) * 0.24;
    let wave = (latitude * 25.0 + (warped.x * 5.0 + time * 0.18).sin() * 0.55).sin();
    let band = smoothstep(-0.78, 0.78, wave);
    let cream = Vector3::new(0.86, 0.68, 0.43);
    let amber = Vector3::new(0.62, 0.24, 0.075);
    let pale = Vector3::new(0.95, 0.84, 0.64);
    let mut color = color_mix(amber, cream, band);
    color = color_mix(color, pale, smoothstep(0.63, 0.82, turbulence) * 0.55);
    let spiral = ((angle * 6.0 - distance * 34.0 + time * 0.45).sin() * 0.5 + 0.5) * influence;
    color = color_mix(
        color,
        color_mix(Vector3::new(0.52, 0.09, 0.035), pale, spiral),
        influence * 0.88,
    );
    let diffuse = fragment.normal.dot(light_direction()).max(0.0);
    color * (0.16 + diffuse * 0.84)
}
