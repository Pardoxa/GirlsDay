use egui::{
    containers::Frame,
    emath::Align,
    {
        Layout, 
        Color32, 
        Vec2,
        Sense
    }, Mesh, plot::{Plot, Legend, PlotPoints, Line},
};
use rand::{SeedableRng, distributions::Uniform, prelude::Distribution};

use crate::random_walker::RandomWalker;

pub struct TemplateApp {

    walker: Option<Vec<RandomWalker>>,
    canvas_size: f32,
    speed: f32,
    current_time: f32,
    zoom: f32,
    old_mesh: Option<Mesh>,
    step_limit: f32,
    seed: f32,
    display_walker_id: f32,
    num_of_walkers: f32
}

impl Default for TemplateApp {
    fn default() -> Self {

        Self {
            // Example stuff:
            zoom: 50.0,
            speed: 1.0,
            current_time: 0.0,
            walker: None,
            canvas_size: 0.5,
            old_mesh: None,
            step_limit: 100000.0,
            seed: 2391.0,
            display_walker_id: 0.0,
            num_of_walkers: 1.0
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);

        // DO NOT SAVE
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            zoom, 
            walker, 
            speed,
            current_time,
            canvas_size,
            old_mesh,
            step_limit,
            seed,
            display_walker_id,
            num_of_walkers
        } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let mut do_steps = 0;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.add(egui::Slider::new(zoom, 20.0..=2000.0).integer().text("Zoom"));
            ui.add(egui::Slider::new(speed, 0.001..=100.0).logarithmic(true).text("Speed"));
            ui.add(egui::Slider::new(canvas_size, 0.0..=1.0).text("Canvas Size"));
            ui.add(egui::Slider::new(step_limit, 1.0..=1e6).text("Step limit"));
            ui.add(egui::Slider::new(seed, 0.0..=1e8).integer().text("Seed"));
            ui.add(egui::Slider::new(num_of_walkers, 1.0..=1e2).integer().text("Number of walkers"));
            if ui.add(egui::Button::new("Create walker")).clicked(){
                let pcg = rand_pcg::Pcg64::seed_from_u64(*seed as u64);
                let seed_iter = Uniform::new_inclusive(0, u64::MAX);

                *walker = Some(
                    seed_iter.sample_iter(pcg)
                        .take(*num_of_walkers as usize)
                        .map(
                            |seed|
                            {
                                RandomWalker::new(seed)
                            }
                        ).collect()
                );
            }

            if let Some(walker) = walker{
                ui.add(egui::Slider::new(display_walker_id, 0.0..=((walker.len()-1) as f32)).integer().text("Display Walker"));
            }

            let old = *current_time as u64;
            *current_time += *speed;
            let new = *current_time as u64;
            do_steps = new - old;


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            //ui.heading("Random Walker");
            //ui.hyperlink("https://github.com/emilk/eframe_template");
            //ui.add(egui::github_link_file!(
            //    "https://github.com/emilk/eframe_template/blob/master/",
            //    "Source code."
            //));

            if let Some(walker_vec) = walker{
                ui.with_layout(
                    Layout::left_to_right(Align::TOP), 
                    |ui|
                    {
                        let idx = (*display_walker_id) as usize;
                        ui.vertical(
                            |ui|
                            {
                                ui.label(format!("Picture of Random Walker {idx}"));

                                Frame::canvas(ui.style())
                                .fill(Color32::BLACK)
                                .show(
                                    ui, 
                                    |ui|
                                    {
                                        ui.ctx().request_repaint();
                                        let min_len = ui.available_size().min_elem();
                                        let desired_canvas = Vec2 { x: min_len, y: min_len } * Vec2{x: *canvas_size, y: *canvas_size};
        
                                        let (response, painter) = ui
                                            .allocate_painter(
                                                desired_canvas, 
                                                Sense::hover()
                                            );
        
                                        let canvas_size = response.rect;
                                        for walker in walker_vec.iter_mut(){
                                            if walker.history.len() < *step_limit as usize{
                                                for _ in 0..do_steps{
                                                    walker.random_step();
                                                }
                                            }
                                        }
                                        
                                        let mesh = if do_steps > 0 || old_mesh.is_none() {
                                            let mesh = crate::animation::calc_mesh(&walker_vec[idx], canvas_size, *zoom);
                                            *old_mesh = Some(mesh.clone());
                                            mesh
                                        } else {
                                            old_mesh.as_ref().unwrap().clone()
                                        };
        
                                        painter.add(mesh);
                                    }
                                );
                            }
                        );
                        
                        
                        let distance: PlotPoints = walker_vec[idx].history.vec
                            .iter()
                            .enumerate()
                            .map(|(index, pos)| [index as f64, ((pos.x * pos.x + pos.y*pos.y) as f64).sqrt()])
                            .collect();

                        ui.vertical_centered(
                            |ui|
                            {
                                ui.label("Distance from Origin");
                                Plot::new("plot_average_etc")
                                .include_x(0.0)
                                .legend(Legend::default())
                                .show(
                                    ui, 
                                    |plot_ui|
                                    {
                                        let line = Line::new(distance).name(format!("walker {idx}"));
                                        plot_ui.line(line);
                                    }
                                );
                            }
                        );

                    }

                );
            }

            

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
