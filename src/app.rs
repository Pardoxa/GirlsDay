use egui::{
    containers::Frame,
    emath::Align,
    {
        Layout, 
        Color32, 
        Vec2,
        Sense
    }, 
    Mesh, 
    plot::*, 
    Button,
    Visuals
};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use rayon::prelude::*;
use crate::animation::{MeshChangeTracker, PerformanceHint};
use crate::random_walker::{RandomWalker, AverageDistance};

#[derive(PartialEq)]
pub enum RadioState{
    NoBias,
    BiasedTowardsOrigin,
    BiasedAwayFromOrigin,
    YourFunction
}

#[derive(PartialEq)]
pub enum LightMode{
    Light,
    Dark
}


pub struct TemplateApp {

    walker: Option<Vec<RandomWalker>>,
    canvas_size: f32,
    speed: f64,
    current_time: f64,
    zoom: f32,
    old_mesh: Option<Mesh>,
    step_limit: u64,
    seed: u64,
    display_walker_id: usize,
    num_of_walkers: usize,
    average: AverageDistance,
    color1: Color32,
    color1_gradient: Color32,
    color2: Color32,
    radio: RadioState,
    strength_of_bias: f64,
    mesh_change_tracker: MeshChangeTracker,
    perfomance_hint: PerformanceHint,
    light_mode: LightMode
}

impl Default for TemplateApp {
    fn default() -> Self {

        Self {
            // Example stuff:
            zoom: 100.0,
            speed: 10.0,
            current_time: 0.0,
            walker: None,
            canvas_size: 0.6,
            old_mesh: None,
            step_limit: 100000,
            seed: 2598,
            display_walker_id: 0,
            num_of_walkers: 10,
            average: AverageDistance::default(),
            color1: Color32::from_rgb(80, 0, 161),
            color1_gradient: Color32::from_rgb(254, 42, 42),
            color2: Color32::DARK_RED,
            radio: RadioState::NoBias,
            strength_of_bias: 0.1,
            mesh_change_tracker: MeshChangeTracker::new(),
            perfomance_hint: PerformanceHint::PrioritizeOptics,
            light_mode: LightMode::Dark
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}
        cc.egui_ctx.set_visuals(Visuals::dark());
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
            num_of_walkers,
            average,
            color1,
            color2,
            color1_gradient,
            radio,
            strength_of_bias,
            mesh_change_tracker,
            perfomance_hint,
            light_mode
        } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let mut do_steps = 0;

        egui::SidePanel::left("side_panel")
            .default_width(300.0)
            .show(ctx, |ui| {

                ui.horizontal(
                    |ui|
                    {
                        match light_mode{
                            LightMode::Dark => {
                                if ui.add(
                                     Button::new("☀").frame(false)
                                 ).on_hover_text("Wechsel in den hellen Modus")
                                 .clicked(){
                                    ui.ctx().set_visuals(Visuals::light());
                                    *light_mode = LightMode::Light;
                                }
                            },
                            LightMode::Light => {
                                if ui.add(
                                     Button::new("🌙").frame(false)
                                 ).on_hover_text("Wechsel in den Dunklen Modus")
                                 .clicked(){
                                    ui.ctx().set_visuals(Visuals::dark());
                                    *light_mode = LightMode::Dark;
                                }
                            }
                        }

                        ui.heading("\t\tEinstellungen");
                    }
                );



            egui::ScrollArea::both().show(
                ui,
                |ui|
                {

                    if ui.add(egui::Slider::new(zoom, 20.0..=3000.0)
                        .integer()
                        .text("Zoom"))
                        .changed(){
                            mesh_change_tracker.request_redraw();
                    }
                    ui.add(egui::Slider::new(speed, 0.001..=2000.0).logarithmic(true).text("Geschwindigkeit"));
                    if ui.add(egui::Slider::new(canvas_size, 0.0..=1.0).text("Bildgröße"))
                        .changed(){
                        mesh_change_tracker.request_redraw();
                    }
                    ui.add(
                        egui::Slider::new(step_limit, 1..=500000)
                            .text("Schritt-limit")
                            .drag_value_speed(10.0)
                    ).on_hover_text("Haben die Walker dieses Schritt-limit erreicht oder überschritten werden sie pausiert");
                    ui.add(egui::Slider::new(seed, 0..=10000)
                        .text("Seed")
                        .drag_value_speed(1.0)
                    );
                    ui.add(egui::Slider::new(num_of_walkers, 1..=200)
                        .text("Anzahl an Walkern")
                        .drag_value_speed(0.5)
                    );
                    if ui.add(egui::Button::new(format!("Erschaffe {num_of_walkers} Walker")))
                        .on_hover_text("Startet die Simulation. Läuft schon eine Simulation so wird sie verworfen und mit den aktuellen Einstellungen wird eine neue gestartet.")
                        .clicked()
                    {
                        let mut pcg = rand_pcg::Pcg64::seed_from_u64(*seed);
                        *current_time = 0.0;
                        let capacity = *step_limit as usize;
                        *walker = None; // Force rust to deallocate the old vectors before allocating new ones!
                        *walker = Some(
                            (0..*num_of_walkers)
                                .map(
                                    |_|
                                    {
                                        RandomWalker::with_capacity_and_rng(
                                            Pcg64::from_rng(&mut pcg).unwrap(), 
                                            capacity
                                        )
                                    }
                                ).collect()
                        );
                        mesh_change_tracker.request_redraw();
                    
                        *average = AverageDistance::default();
                    }
                    ui.horizontal(
                        |ui|
                        {
                            ui.label("Farbe 1");
                            egui::color_picker::color_edit_button_srgba(ui, color1, egui::color_picker::Alpha::Opaque);
                        }
                    );
                    ui.horizontal(
                        |ui|
                        {
                            ui.label("Farbe 2");
                            egui::color_picker::color_edit_button_srgba(ui, color1_gradient, egui::color_picker::Alpha::Opaque);
                        }
                    );
                    ui.horizontal(
                        |ui|
                        {
                            ui.label("Farbe 3");
                            egui::color_picker::color_edit_button_srgba(ui, color2, egui::color_picker::Alpha::Opaque);
                        }
                    );
                    ui.radio_value(radio, RadioState::NoBias, "Normaler Random Walk")
                        .on_hover_text("Ist diese Option ausgewählt wird ein 'normaler' random walk ausgeführt. D.h. die Wahrscheinlichkeit für den Walker nach oben, unten, rechts oder links zu gehen ist identisch.");
                    ui.radio_value(radio, RadioState::BiasedAwayFromOrigin, "Bias - weg vom Ursprung")
                        .on_hover_text("Ist diese Option ausgewählt wird ein gebiaster random walk ausgeführt. Schritte die vom ursprung wegführen werden preferiert. Wie stark diese Präferenz ist hängt vom Bias ab.");
                    ui.radio_value(radio, RadioState::BiasedTowardsOrigin, "Bias - zum Ursprung hin")
                        .on_hover_text("Ist diese Option ausgewählt wird ein gebiaster random walk ausgeführt. Schritte die zum ursprung hinführen werden preferiert. Wie stark diese Präferenz ist hängt vom Bias ab.");
                    ui.radio_value(radio, RadioState::YourFunction, "Deine Funktion")
                        .on_hover_text("Du hast eine eigene Idee für einen Random Walker? Perfekt. Du kannst sie implementieren und hier auswählen");
                    ui.add(
                        egui::Slider::new(strength_of_bias, 0.0..=0.5)
                        .logarithmic(true)
                        .text("Bias")
                        .smallest_positive(0.0005)
                    );
                
                    if let Some(walker) = walker{
                        if ui
                            .add(egui::Slider::new(display_walker_id, 0..=(walker.len()-1))
                            .drag_value_speed(0.5)
                            .text("Walker im Fokus"))
                            .changed(){
                            
                            mesh_change_tracker.request_redraw();
                        }
                    }

                    ui.radio_value(perfomance_hint, PerformanceHint::PrioritizeOptics, "Priorität: Optik")
                        .on_hover_text("Dies wird das Bild bei jedem Schritt sanft umfärben und alle Schritte für das Diagramm verwenden");
                    ui.radio_value(perfomance_hint, PerformanceHint::PrioritizePerformance, "Priorität: Leistung")
                        .on_hover_text("Dies wird das Bild nur gelegentlich umfärben und nur jeden 100. Punkt im Diagramm verwenden");
                
                    let old = *current_time as u64;
                    *current_time += *speed;
                    let new = *current_time as u64;
                    do_steps = new - old;

                    egui::warn_if_debug_build(ui);
                }
            );
            
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
                        let idx = *display_walker_id;
                        ui.vertical(
                            |ui|
                            {
                                ui.label(format!("Walker {idx}"));

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

                                        match radio{
                                            RadioState::NoBias => {
                                                walker_vec.par_iter_mut()
                                                    .for_each(
                                                        |walker|
                                                        {
                                                            if walker.history.len() < *step_limit as usize{
                                                                for _ in 0..do_steps{
                                                                    walker.random_step();
                                                                }
                                                            }
                                                        }
                                                    );
                                            },
                                            _ => {
                                                let step_fun = match radio {
                                                    RadioState::BiasedAwayFromOrigin => RandomWalker::random_step_biased_away,
                                                    RadioState::BiasedTowardsOrigin => RandomWalker::random_step_biased_to_origin,
                                                    RadioState::YourFunction => RandomWalker::your_step_function,
                                                    _ => unreachable!()
                                                };

                                                walker_vec.par_iter_mut()
                                                    .for_each(
                                                        |walker|
                                                        {
                                                            if walker.history.len() < *step_limit as usize{
                                                                for _ in 0..do_steps{
                                                                    step_fun(walker, *strength_of_bias);
                                                                }
                                                            }
                                                        }
                                                    );
                                            }
                                        };

                                        
                                        if do_steps > 0 && average.average_distance_plot_data.len() < *step_limit as usize {
                                            average.update_on_step_of_walkers(do_steps as usize, walker_vec);
                                            if perfomance_hint == &PerformanceHint::PrioritizeOptics{
                                                mesh_change_tracker.request_redraw();   
                                            }
                                        }
                                        
                                        let mesh = if mesh_change_tracker.check_if_needs_redraw(*speed, *perfomance_hint) || old_mesh.is_none() {
                                            let mesh = crate::animation::calc_mesh(
                                                &walker_vec[idx], 
                                                canvas_size, 
                                                *zoom,
                                                *color1,
                                                *color1_gradient,
                                                *color2
                                            );
                                            let total_steps = walker_vec[idx].history.len();
                                            mesh_change_tracker.redraw_finished(total_steps);
                                            *old_mesh = Some(mesh.clone());
                                            mesh
                                        } else {
                                            let saved_mesh = old_mesh.as_mut().unwrap();
                                            let old_steps = mesh_change_tracker.get_current_step();
                                            let new_steps = walker_vec[idx].history.len();
                                            if old_steps != new_steps{
                                                crate::animation::update_mesh(
                                                    saved_mesh, 
                                                    old_steps, 
                                                    &walker_vec[idx], 
                                                    canvas_size, 
                                                    *zoom, 
                                                    *color2
                                                );
                                                mesh_change_tracker.new_steps(new_steps);
                                                if new_steps >= *step_limit as usize {
                                                    mesh_change_tracker.request_redraw();
                                                }
                                            }
                                            
                                            saved_mesh.clone()
                                        };
        
                                        painter.add(mesh);
                                    }
                                );
                            }
                        );

                        let max_reached = walker_vec[idx].history.len();

                        let step_size = max_reached as f64 / 1000.0;
                        let factor = std::f64::consts::PI.sqrt() / 2.0;

                        let analytical: Vec<_> = (0..1000_u32)
                            .map(
                                |i|
                                {
                                    let x = (i as f64) * step_size;
                                    let y = x.sqrt() * factor;
                                    [x,y]
                                }
                            ).collect();
                        

                        
                        
                        let distance: Vec<PlotPoint> = match *perfomance_hint{
                            PerformanceHint::PrioritizeOptics => {
                                walker_vec[idx]
                                    .history
                                    .distance_from_origin
                                    .par_iter()
                                    .enumerate()
                                    .map(|(index, dist)| PlotPoint { x: index as f64, y: *dist as f64})
                                    .collect()
                            },
                            _ => {
                                walker_vec[idx]
                                    .history
                                    .distance_from_origin
                                    .par_iter()
                                    .enumerate()
                                    .step_by(100)
                                    .map(|(index, dist)| PlotPoint { x: index as f64, y: *dist as f64 })
                                    .collect()
                            }
                        };

                        ui.vertical_centered(
                            |ui|
                            {
                                ui.label("Abstand vom Urspurng");

                                let hight = ui.available_height();
                                Plot::new("plot_average_etc")
                                .include_x(0.0)
                                .legend(Legend::default())
                                .height(hight - 25.0)
                                .show(
                                    ui, 
                                    |plot_ui|
                                    {
                                        let line = Line::new(PlotPoints::Owned(distance))
                                            .name(format!("walker {idx}"))
                                            .color(*color2);
                                        plot_ui.line(line);
                                        

                                        let average_distance = match *perfomance_hint
                                        {
                                            PerformanceHint::PrioritizeOptics => {
                                                average
                                                    .cloned_average()
                                            },
                                            _ => {
                                                average.get_approximation()
                                            }
                                        };

                                        let line = Line::new(PlotPoints::Owned(average_distance)).name("average");
                                        plot_ui.line(line);
                                        let analytical_line = Line::new(analytical).name("analytical Results");
                                        plot_ui.line(analytical_line);
                                        
                                        //let y = plot_ui.plot_bounds().max()[1];
                                        //let x = plot_ui.plot_bounds().max()[0];
                                        //
                                        //let text = egui::plot::Text::new(PlotPoint { x: x / 20.0, y: y / 2.0 }, "d")
                                        //    .anchor(Align2::LEFT_CENTER);
                                        //plot_ui.text(text);
                                    }
                                );
                                ui.label("Steps");
                            }
                        );

                    }

                );
            }
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
