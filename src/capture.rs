use crate::celestial_shaders::CelestialKind;
use crate::framebuffer::Framebuffer;
use crate::load_objects::load_obj;
use crate::render::Renderer;
use crate::shaders::FragmentShader;
use crate::uniforms::Uniforms;
use raylib::prelude::{Color, Matrix, Vector3};

pub fn generate() {
    std::fs::create_dir_all("capturas").expect("No se pudo crear capturas/");
    let mesh = load_obj("assets/sphere.obj").expect("No se pudo cargar assets/sphere.obj");
    let camera_pos = Vector3::new(0.0, 30.0, -520.0);
    let uniforms = Uniforms {
        model_matrix: Matrix::scale(190.0, 190.0, 190.0),
        camera_pos,
        camera_target: Vector3::zero(),
        camera_up: Vector3::new(0.0, 1.0, 0.0),
    };

    for (name, kind) in [
        ("estrella", CelestialKind::Star),
        ("planeta_rocoso", CelestialKind::Rocky),
        ("gigante_gaseoso", CelestialKind::GasGiant),
    ] {
        let mut framebuffer = Framebuffer::new(600, 600, Color::new(3, 5, 12, 255));
        Renderer::default().render(
            &mut framebuffer,
            &uniforms,
            &mesh,
            FragmentShader::Celestial {
                kind,
                time: 12.0,
                camera_pos,
            },
        );
        framebuffer
            .color_buffer
            .export_image(&format!("capturas/{name}.png"));
    }
}
