use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;

use eframe::Frame;
use egui::{ColorImage, Context, Image, Key, KeyboardShortcut, Modifiers, Pos2, Rect, Slider, TextureHandle, Visuals};
use egui_keybind::Shortcut;
use nalgebra::{Perspective3, Point3, Translation3, UnitDualQuaternion, UnitQuaternion};
use zune_jpeg::JpegDecoder;

struct Controls {
    roll_left: Shortcut,
    roll_right: Shortcut,
    look_up: Shortcut,
    look_down: Shortcut,
    look_left: Shortcut,
    look_right: Shortcut,

    move_left: Shortcut,
    move_right: Shortcut,
    move_up: Shortcut,
    move_down: Shortcut,
    move_forward: Shortcut,
    move_back: Shortcut,
}

pub struct UMAPViewer {
    loc_rot: UnitDualQuaternion<f32>,

    items: Vec<Point3<f32>>,
    tex: TextureHandle,

    image_scale: f32,
    fov: f32,

    move_speed: f32,
    turn_speed: f32,

    controls: Controls,
}

impl UMAPViewer {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals {
            dark_mode: true,
            ..Default::default()
        });

        // Cube
        let dummy_items = vec![
            Point3::new(0., 0., 0.),
            Point3::new(0., 0., 1.),
            Point3::new(0., 1., 0.),
            Point3::new(0., 1., 1.),
            Point3::new(1., 0., 0.),
            Point3::new(1., 0., 1.),
            Point3::new(1., 1., 0.),
            Point3::new(1., 1., 1.),
            Point3::new(0.25, 0.25, 0.25),
            Point3::new(0.25, 0.25, 0.75),
            Point3::new(0.25, 0.75, 0.25),
            Point3::new(0.25, 0.75, 0.75),
            Point3::new(0.75, 0.25, 0.25),
            Point3::new(0.75, 0.25, 0.75),
            Point3::new(0.75, 0.75, 0.25),
            Point3::new(0.75, 0.75, 0.75),
        ];

        // Axes
        // let mut dummy_items = vec![];
        // for i in 0..11 {
        //     let pos = (i - 5) as f32;
        //     dummy_items.push(Point3::new(pos, 0., 0.));
        //     dummy_items.push(Point3::new(0., pos, 0.));
        //     dummy_items.push(Point3::new(0., 0., pos));
        // }

        // Dummy point texture
        let file = File::open(r"E:\datasets\tcga\images\hcmi_cmdc_tiles\HCM-BROD-0557-C43-06A-01-S2-HE_130.jpg").unwrap();
        let mut decoder = JpegDecoder::new(BufReader::new(file));
        let bytes = decoder.decode().unwrap();
        let img = ColorImage::from_rgb([512, 512], &bytes);

        let tex = cc.egui_ctx.load_texture("my-image", img, Default::default());

        Self {
            loc_rot: UnitDualQuaternion::from_parts(
                Translation3::new(0., 0., -5.),
                UnitQuaternion::identity(),
            ),
            items: dummy_items,
            tex,
            image_scale: 1.,
            fov: PI / 2.,

            move_speed: 0.1,
            turn_speed: 0.02,
            controls: Controls {
                roll_left: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::Q)), None),
                roll_right: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::E)), None),
                look_up: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::ArrowUp)), None),
                look_down: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::ArrowDown)), None),
                look_left: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::ArrowLeft)), None),
                look_right: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::ArrowRight)), None),
                move_left: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::A)), None),
                move_right: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::D)), None),
                move_up: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::Space)), None),
                move_down: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::X)), None),
                move_forward: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::W)), None),
                move_back: Shortcut::new(Some(KeyboardShortcut::new(Modifiers::NONE, Key::S)), None),
            },
        }
    }

    fn handle_movement(&mut self, ctx: &Context) {
        let mut needs_repaint = false;
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.roll_left.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_rotation(UnitQuaternion::from_euler_angles(0., 0., -self.turn_speed));
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.look_up.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_rotation(UnitQuaternion::from_euler_angles(-self.turn_speed, 0., 0.));
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.look_left.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_rotation(UnitQuaternion::from_euler_angles(0., -self.turn_speed, 0.));
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.roll_right.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_rotation(UnitQuaternion::from_euler_angles(0., 0., self.turn_speed));
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.look_down.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_rotation(UnitQuaternion::from_euler_angles(self.turn_speed, 0., 0.));
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.look_right.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_rotation(UnitQuaternion::from_euler_angles(0., self.turn_speed, 0.));
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }

        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.move_left.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_parts(Translation3::new(self.move_speed, 0., 0.), UnitQuaternion::identity());
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.move_down.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_parts(Translation3::new(0., self.move_speed, 0.), UnitQuaternion::identity());
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.move_forward.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_parts(Translation3::new(0., 0., self.move_speed), UnitQuaternion::identity());
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.move_right.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_parts(Translation3::new(-self.move_speed, 0., 0.), UnitQuaternion::identity());
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.move_up.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_parts(Translation3::new(0., -self.move_speed, 0.), UnitQuaternion::identity());
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
        if ctx.input_mut(|i| i.keys_down.contains(&self.controls.move_back.keyboard().unwrap().logical_key)) {
            let rot = UnitDualQuaternion::from_parts(Translation3::new(0., 0., -self.move_speed), UnitQuaternion::identity());
            self.loc_rot = rot * self.loc_rot;
            needs_repaint = true;
        }
    }
}


impl eframe::App for UMAPViewer {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::Window::new("controls").show(ctx, |ui| {
            ui.label("Config");
            ui.add(Slider::new(&mut self.image_scale, 0.0..=5.0).text("Image Scale"));
            ui.add(Slider::new(&mut self.fov, 0.0..=PI).text("FOV"));
            ui.add(Slider::new(&mut self.turn_speed, 0.001..=0.1).text("Turning speed"));
            ui.add(Slider::new(&mut self.move_speed, 0.001..=1.).text("Move speed"));
        });

        self.handle_movement(ctx);


        let mut rect = Default::default();
        let mut points = vec![];
        egui::CentralPanel::default().show(ctx, |ui| {
            rect = ui.available_size();
            let camera = Perspective3::new(rect.x / rect.y, self.fov, 0.001, 10000.0);
            let transform = camera.as_matrix();

            // Transform points
            for point in &self.items {
                let p_camera = self.loc_rot.transform_point(point);
                let p_clip = transform.transform_point(&p_camera);

                let is_visible = (-1.0..1.0).contains(&p_clip.z)
                    && (-1.0..1.0).contains(&p_clip.y)
                    && (-1.0..1.0).contains(&p_clip.x);

                let p_screen = Point3::new(
                    rect.x * (p_clip.x + 1.) / 2.,
                    rect.y * (p_clip.y + 1.) / 2.,
                    0.,
                );


                points.push(format!("{} | {} -> {} -> {} -> {}", is_visible, point, p_camera, p_clip, p_screen));

                // Show points
                if is_visible {
                    let distance = p_camera.coords.norm();

                    let screen_height = (camera.fovy() / 2.).tan() * distance;
                    let image_scale = self.image_scale / screen_height;

                    let sized_image = egui::load::SizedTexture::new(self.tex.id(), (self.tex.size_vec2() * image_scale).ceil());
                    let image = Image::from_texture(sized_image);
                    let offset = image.size().unwrap() / 2.;

                    let loc = Rect::from_min_size(
                        Pos2::new(p_screen.x - offset.x, rect.y - p_screen.y - offset.y), // Flip Y since we're upside down
                        image.size().unwrap(),
                    );
                    ui.put(loc, image);

                    // Debug coordinate
                    // ui.put(loc.expand(100.).translate(Vec2::new(0., 10.)), egui::Label::new(format!("{:?}", point)));
                }
            }
        });


        egui::Window::new("Debug_coords").default_open(false).show(ctx, |ui| {
            ui.label(format!("Area: {:?}", rect));

            ui.label(format!("LocRot: {:?}", self.loc_rot));

            for p in points {
                ui.label(p);
            }
        });

        ctx.request_repaint()
    }
}