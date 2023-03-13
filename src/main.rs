// #![windows_subsystem = "windows"]

use macroquad::prelude::*;
use egui_macroquad::*;

fn config() -> Conf {
    return Conf {
        window_title: String::from("Nuclear Fission"),
        fullscreen: false,
        ..Default::default()
    };
}

struct Camera {
    x: f32,
    y: f32,
    zoom_scale: f32,
    speed: f32,
}

struct Uranium235 {
    x: f32,
    y: f32,
}

struct Neutron {
    x: f32,
    y: f32,
}

#[macroquad::main(config())]
async fn main() {
    let mut camera = Camera {
        x: 0.0,
        y: 0.0,
        zoom_scale: 10.0,
        speed: 300.0,
    };

    let uranium235_texture = load_texture("./assets/Uranium235.png").await.unwrap();

    let mut uranium235_array = Vec::<Uranium235>::new();
    let mut neutron_array = Vec::<Neutron>::new();

    generate_uranium(&mut uranium235_array, 2000);

    loop {
        clear_background(BLACK);

        set_default_camera();

        fetch_movement(&mut camera);
        update_camera(&camera);
        render_neutrons(&neutron_array);
        render_uranium(&uranium235_texture, &uranium235_array);

        if is_key_pressed(KeyCode::N) {
            neutron_array.push(Neutron {x: 0.0, y: 0.0});
        }

        egui_macroquad::ui(|egui_ctx| {
            let window = egui::Window::new("Info");
            window.show(egui_ctx, |ui| {
                ui.colored_label(egui::Color32::WHITE, "Test");
                ui.label(format!("Uranium 235: {}", uranium235_array.len()));
                if ui.add(egui::Button::new("Generate")).clicked() {
                    uranium235_array.clear();
                    generate_uranium(&mut uranium235_array, 2000);
                }
                // ui.allocate_space(ui.available_size());
                ui.allocate_space((ui.available_width(), 0.0).into());
            });
        });
        egui_macroquad::draw();
        next_frame().await;
    }
}

fn update_camera(camera: &Camera) -> Camera2D {
    let camera = Camera2D {
        zoom: vec2(
            camera.zoom_scale * 0.0001,
            (screen_width() / screen_height()) * camera.zoom_scale * 0.0001
        ),
        target: (camera.x, camera.y).into(),
        ..Default::default()
    };
    set_camera(&camera);
    return camera;
}

fn render_uranium(uranium235_texture: &Texture2D, uranium235_array: &Vec<Uranium235>) {
    draw_poly(0.0, 0.0, 100, 1000.0, 0.0, GRAY);
    for i in 0..uranium235_array.len() {
        render_particle(uranium235_texture, uranium235_array[i].x, uranium235_array[i].y);
    }
}

fn fetch_movement(camera: &mut Camera) {
    if is_key_down(KeyCode::D) {
        camera.x += camera.speed / camera.zoom_scale;
    }
    if is_key_down(KeyCode::A) {
        camera.x -= camera.speed / camera.zoom_scale;
    }
    if is_key_down(KeyCode::S) {
        camera.y -= camera.speed / camera.zoom_scale;
    }
    if is_key_down(KeyCode::W) {
        camera.y += camera.speed / camera.zoom_scale;
    }
    if is_key_down(KeyCode::Space) {
        if camera.zoom_scale >= 5.0 {
            camera.zoom_scale *= 0.97;
        }
    }
    if is_key_down(KeyCode::LeftShift) {
        if camera.zoom_scale <= 5000.0 {
            camera.zoom_scale *= 1.03;
        }
    }
}

fn render_particle(texture: &Texture2D, x: f32, y: f32) {
    draw_texture_ex(*texture, x - 6.5, y - 6.5, WHITE, DrawTextureParams {
        dest_size: Some((13.0, 13.0).into()),
        flip_x: false,
        flip_y: true,
        pivot: None,
        rotation: 0.0,
        source: None,
    });
}

fn render_neutrons(n_array: &Vec::<Neutron>) {
    for i in 0..n_array.len() {
        render_neutron(n_array[i].x, n_array[i].y);
    }
}

fn render_neutron(x: f32, y: f32) {
    draw_circle(x, y, 1.0, WHITE);
}

fn generate_uranium(u_array: &mut Vec::<Uranium235>, number_of_particles: usize) {
    push_uranium_to_array(u_array, (number_of_particles - u_array.len()));
    remove_collisions(u_array);
    while u_array.len() < number_of_particles {
        push_uranium_to_array(u_array, number_of_particles - u_array.len());
        remove_collisions(u_array);
    }
}

fn push_uranium_to_array(u_array: &mut Vec::<Uranium235>, nop: usize) {
    let size = 1000.0;
    for _ in 0..nop {
        let mut x = 1000.0;
        let mut y = 1000.0;
        while x * x + y * y > 970000.0 {
            x = rand::gen_range(-size, size);
            y = rand::gen_range(-size, size);
        }
        u_array.push(Uranium235 { x, y });
    }
}

impl std::fmt::Debug for Uranium235 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "[{}, {}]", self.x, self.y);
    }
}

fn remove_collisions(u_array: &mut Vec::<Uranium235>) {
    u_array.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    for i in (0..u_array.len()).rev() {
        for j in (0..i).rev() {
            if u_array[i].x - u_array[j].x > 13.0 {break}
            if fast_distance(u_array[i].x, u_array[i].y, u_array[j].x, u_array[j].y) > 169.0 {continue}
            u_array.remove(j);
            break;
        }
    }
}

fn fast_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)
}