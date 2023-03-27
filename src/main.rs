// #![windows_subsystem = "windows"] // This line is used to hide console window when compiling to windows

/*
PS
This code should not be used for educational purposes,
it just works, thats it.
*/

use macroquad::prelude::*;
use egui_macroquad::*;
use egui_macroquad::egui::RichText;

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
    vel_x: f32,
    vel_y: f32,
}

impl Neutron {
    fn new(x: f32, y: f32) -> Self {
        let angle: f32 = rand::gen_range(-1.0, 1.0) * std::f32::consts::PI;
        Neutron {
            x,
            y,
            vel_x: angle.sin() / 10.0,
            vel_y: angle.cos() / 10.0,
        }
    }

    fn update(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    fn fast_distance_from_0_0(&self) -> f32 {
        fast_distance(0.0, 0.0, self.x, self.y)
    }
}

struct UraniumFissionProduct {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    has_neutron: bool,
}

impl UraniumFissionProduct {
    fn new(x: f32, y: f32, has_neutron: bool) -> Self {
        let angle: f32 = rand::gen_range(-1.0, 1.0) * std::f32::consts::PI;
        let speed = rand::gen_range(0.05, 0.3);
        UraniumFissionProduct {
            x,
            y,
            vel_x: (angle.sin() / 10.0) * speed,
            vel_y: (angle.cos() / 10.0) * speed,
            has_neutron,
        }
    }

    fn update(&mut self, n_array: &mut Vec<Neutron>) {
        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.has_neutron && rand::gen_range(0, 10000) == 0 {
            self.has_neutron = false;
            n_array.push(Neutron::new(self.x, self.y));
        }
    }

    fn fast_distance_from_0_0(&self) -> f32 {
        fast_distance(0.0, 0.0, self.x, self.y)
    }
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
    let uranium_fission_product_texture = load_texture(
        "./assets/Uranium235-fission-product.png"
    ).await.unwrap();

    let mut uranium235_array = Vec::<Uranium235>::new();
    let mut neutron_array = Vec::<Neutron>::new();
    let mut ufp_array = Vec::<UraniumFissionProduct>::new(); //uranium fission product

    let mut simulation_speed = 1.0;
    let mut paused = false;
    let mut ufp_opacity = 50.0;
    let mut uranium_to_generate: usize = 1000;
    reset_simulation(
        &mut uranium235_array,
        &mut neutron_array,
        &mut ufp_array,
        uranium_to_generate
    );

    loop {
        clear_background(BLACK);

        set_default_camera();

        fetch_movement(&mut camera);
        let main_camera = update_camera(&camera);
        draw_poly(0.0, 0.0, 100, 1000.0, 0.0, GRAY);
        render_neutrons(&neutron_array);
        render_uranium_fission_products(
            &uranium_fission_product_texture,
            &ufp_array,
            ufp_opacity / 75.0
        );
        render_uranium(&uranium235_texture, &uranium235_array);

        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }
        if is_key_down(KeyCode::Left) {
            simulation_speed -= 0.1;
            if simulation_speed < 0.1 {
                simulation_speed = 0.1;
            }
        }
        if is_key_down(KeyCode::Right) {
            simulation_speed += 0.1;
            if simulation_speed > 10.0 {
                simulation_speed = 10.0;
            }
        }

        if is_key_pressed(KeyCode::N) {
            let (mouse_x, mouse_y) = main_camera.screen_to_world(mouse_position().into()).into();
            neutron_array.push(Neutron::new(mouse_x, mouse_y));
        }
        if !paused {
            for _ in 0..(simulation_speed * 10.0) as usize {
                update_simulation(&mut neutron_array, &mut uranium235_array, &mut ufp_array);
            }
        } else {
            if is_key_pressed(KeyCode::L) {
                update_simulation(&mut neutron_array, &mut uranium235_array, &mut ufp_array);
            }
        }

        egui_macroquad::ui(|egui_ctx| {
            let window = egui::Window::new("Settings").default_open(true).resizable(false);
            window.show(egui_ctx, |ui| {
                ui.label(format!("Press P to {} the simulation\n", paused_str(paused)));
                ui.label(format!("Uranium 235      : {}", uranium235_array.len())).on_hover_text(
                    "Current amount of U235 in simulation"
                );
                ui.label(format!("Neutrons            : {}", neutron_array.len())).on_hover_text(
                    "Current amount of Neutrons in simulation"
                );
                ui.label(format!("Fission Products: {}", ufp_array.len())).on_hover_text(
                    "Current amount of FP in simulation"
                );
                ui.label("");
                ui.add(
                    egui::Slider::new(&mut simulation_speed, 0.1..=10.0).text("Simulation Speed")
                ).on_hover_text(
                    "You can also use Arrow Keys on your keyboard \n\n You also can press P to pause/resume at any time"
                );
                ui.add(
                    egui::Slider
                        ::new(&mut ufp_opacity, 0.0..=100.0)
                        .text("FP Opacity")
                        .step_by(1.0)
                ).on_hover_text("Uranium Fission Products Opacity (%)");
                ui.label("");
                ui.add(
                    egui::Slider
                        ::new(&mut uranium_to_generate, 10..=2000)
                        .text("U235 amount")
                        .step_by(1.0)
                ).on_hover_text("Amount of Uranium 235 to generate");
                if
                    ui
                        .add(egui::Button::new("Generate"))
                        .on_hover_text("Reset simulation with specified amount of U235")
                        .clicked()
                {
                    reset_simulation(
                        &mut uranium235_array,
                        &mut neutron_array,
                        &mut ufp_array,
                        uranium_to_generate
                    );
                }
                ui.label("\n\n");
                ui.label(RichText::new("HELP").strong()).on_hover_text(
                    format!(
                        "{}\n\n{}\n\n{}\n\n{}\n\n{}",
                        "W A S D    ->   Camera Movement",
                        "Shift/Space    ->   Zoom In/Out",
                        "N    ->   Spawn new neutron at mouse cursor",
                        "Arrow Keys    ->   Change speed of the simulation",
                        "P    -> Pause/Resume the simulation"
                    )
                );
            });
        });
        egui_macroquad::draw();
        next_frame().await;
    }
}

fn paused_str(paused: bool) -> String {
    if paused {
        return "resume".to_string();
    } else {
        return "pause".to_string();
    }
}

fn reset_simulation(
    uranium235_array: &mut Vec<Uranium235>,
    neutron_array: &mut Vec<Neutron>,
    ufp_array: &mut Vec<UraniumFissionProduct>,
    uranium_to_generate: usize
) {
    uranium235_array.clear();
    neutron_array.clear();
    ufp_array.clear();
    generate_uranium(uranium235_array, uranium_to_generate);
}

fn update_simulation(
    neutron_array: &mut Vec<Neutron>,
    uranium235_array: &mut Vec<Uranium235>,
    ufp_array: &mut Vec<UraniumFissionProduct>
) {
    for i in (0..neutron_array.len()).rev() {
        neutron_array[i].update();
        if neutron_array[i].fast_distance_from_0_0() > 1_000_000.0 {
            neutron_array.remove(i);
        }
    }
    for i in (0..ufp_array.len()).rev() {
        ufp_array[i].update(neutron_array);
        if ufp_array[i].fast_distance_from_0_0() > 1_000_000.0 {
            ufp_array.remove(i);
        }
    }
    fetch_collisions(neutron_array, uranium235_array, ufp_array);
}

fn update_camera(camera: &Camera) -> Camera2D {
    let main_camera = Camera2D {
        zoom: vec2(
            camera.zoom_scale * 0.0001,
            (screen_width() / screen_height()) * camera.zoom_scale * 0.0001
        ),
        target: (camera.x, camera.y).into(),
        ..Default::default()
    };
    set_camera(&main_camera);
    return main_camera;
}

fn render_uranium(uranium235_texture: &Texture2D, uranium235_array: &Vec<Uranium235>) {
    for i in 0..uranium235_array.len() {
        render_particle(uranium235_texture, uranium235_array[i].x, uranium235_array[i].y, WHITE);
    }
}

fn render_uranium_fission_products(
    uranium_fission_product_texture: &Texture2D,
    ufp_array: &Vec<UraniumFissionProduct>,
    ufp_opacity: f32
) {
    for i in 0..ufp_array.len() {
        if ufp_array[i].has_neutron {
            render_particle(
                uranium_fission_product_texture,
                ufp_array[i].x + rand::gen_range(-0.5, 0.5),
                ufp_array[i].y + rand::gen_range(-0.5, 0.5),
                Color::new(1.0, 1.0, 1.0, ufp_opacity)
            );
        } else {
            render_particle(
                uranium_fission_product_texture,
                ufp_array[i].x,
                ufp_array[i].y,
                Color::new(1.0, 1.0, 1.0, ufp_opacity - 0.5)
            );
        }
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

fn render_particle(texture: &Texture2D, x: f32, y: f32, color: Color) {
    draw_texture_ex(*texture, x - 6.5, y - 6.5, color, DrawTextureParams {
        dest_size: Some((13.0, 13.0).into()),
        flip_x: false,
        flip_y: true,
        pivot: None,
        rotation: 0.0,
        source: None,
    });
}

fn render_neutrons(n_array: &Vec<Neutron>) {
    for i in 0..n_array.len() {
        draw_poly(n_array[i].x, n_array[i].y, 8, 1.0, 0.0, WHITE);
    }
}

fn generate_uranium(u_array: &mut Vec<Uranium235>, number_of_particles: usize) {
    push_uranium_to_array(u_array, number_of_particles - u_array.len());
    remove_collisions(u_array);
    while u_array.len() < number_of_particles {
        push_uranium_to_array(u_array, number_of_particles - u_array.len());
        remove_collisions(u_array);
    }
}

fn push_uranium_to_array(u_array: &mut Vec<Uranium235>, number_of_particles: usize) {
    let size = (number_of_particles + u_array.len()) as f32;
    let smart_size = (size / 3.1415926535).sqrt() * 40.0;
    for _ in 0..number_of_particles {
        let mut x = f32::INFINITY;
        let mut y = f32::INFINITY;
        while x * x + y * y > smart_size * 970.0 {
            x = rand::gen_range(-1000.0, 1000.0);
            y = rand::gen_range(-1000.0, 1000.0);
        }
        u_array.push(Uranium235 { x, y });
    }
}

impl std::fmt::Debug for Uranium235 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "[{}, {}]", self.x, self.y);
    }
}

fn remove_collisions(u_array: &mut Vec<Uranium235>) {
    u_array.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    for i in (0..u_array.len()).rev() {
        for j in (0..i).rev() {
            if u_array[i].x - u_array[j].x > 13.0 {
                break;
            }
            if fast_distance(u_array[i].x, u_array[i].y, u_array[j].x, u_array[j].y) > 169.0 {
                continue;
            }
            u_array.remove(j);
            break;
        }
    }
}

fn fast_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
}

fn fetch_collisions(
    n_array: &mut Vec<Neutron>,
    u_array: &mut Vec<Uranium235>,
    ufp_array: &mut Vec<UraniumFissionProduct>
) {
    if n_array.len() == 0 || u_array.len() == 0 {
        return;
    }
    u_array.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    n_array.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    let mut u_index = u_array.len() - 1;
    let mut ofset;
    for n in (0..n_array.len()).rev() {
        while u_array[u_index].x - 6.5 > n_array[n].x {
            if u_index == 0 {
                return;
            }
            u_index -= 1;
        }
        ofset = 0;
        while u_index >= ofset && u_array[u_index - ofset].x + 6.5 > n_array[n].x {
            if
                fast_distance(
                    u_array[u_index - ofset].x,
                    u_array[u_index - ofset].y,
                    n_array[n].x,
                    n_array[n].y
                ) < 42.25
            {
                n_array.remove(n);
                for _ in 0..2 {
                    n_array.push(
                        Neutron::new(u_array[u_index - ofset].x, u_array[u_index - ofset].y)
                    );
                }
                ufp_array.push(
                    UraniumFissionProduct::new(
                        u_array[u_index - ofset].x,
                        u_array[u_index - ofset].y,
                        true
                    )
                );
                ufp_array.push(
                    UraniumFissionProduct::new(
                        u_array[u_index - ofset].x,
                        u_array[u_index - ofset].y,
                        false
                    )
                );

                u_array.remove(u_index - ofset);
                if u_index == 0 {
                    if u_array.len() == 0 {
                        return;
                    }
                    break;
                }
                u_index -= 1;
                break;
            }
            ofset += 1;
        }
    }
}