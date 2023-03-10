use eframe::epaint::*;

use crate::random_walker::RandomWalker;

pub fn calc_mesh(
    walker: &RandomWalker,
    canvas_size: Rect,
    zoom: f32,
    color: Color32,
    col1_grad: Color32,
    color2: Color32
) -> Mesh
{
    let mut mesh = Mesh::default();

    let diff = canvas_size.max.to_vec2()
        - canvas_size.min.to_vec2();

    let scale = diff / zoom;

    let origin = 
        canvas_size.min.to_vec2()
        + diff * Vec2 { x: 0.5, y: 0.5 };

    let width_diff = scale * Vec2 { x: 1.0, y: 1.0 };

    let mut add_to_mesh = |x, y, col| {

        let x = origin.x + scale.x * x as f32;
        let y = origin.y + scale.y * y as f32;

        let x_max = x + width_diff.x;
        let y_max = y + width_diff.y;

        let min = Pos2{
            x,
            y
        };

        let max = Pos2{
            x: x_max,
            y: y_max
        };

        mesh.add_colored_rect(
            Rect{
                min, 
                max
            }, 
            col
        );
    };

    let total = 1.0 / (walker.history.len() as f32);

    let red_dist = (col1_grad.r() as i16 - color.r() as i16) as f32;
    let blue_dist = (col1_grad.b() as i16 - color.b() as i16) as f32;
    let green_dist = (col1_grad.g() as i16 - color.g() as i16) as f32;

    let r = color.r() as f32;
    let b = color.b() as f32;
    let g = color.g() as f32;


    for (i, pos) in walker.history.vec.iter().enumerate()
    {
        let p = i as f32 * total;
        let red: u8 = (r + p * red_dist) as u8;
        let green = (g + p * green_dist) as u8;
        let blue = (b + p * blue_dist) as u8;

        let col = Color32::from_rgb(red, green, blue);

        add_to_mesh(
            pos.x,
            pos.y,
            col
        );
    }

    add_to_mesh(
        walker.ort.x,
        walker.ort.y,
        color2
    );

    mesh

}

#[allow(clippy::too_many_arguments)]
pub fn update_mesh(
    mesh: &mut Mesh,
    old_total: usize,
    walker: &RandomWalker,
    canvas_size: Rect,
    zoom: f32,
    color: Color32,
    col1_grad: Color32,
    color2: Color32
)
{

    let diff = canvas_size.max.to_vec2()
        - canvas_size.min.to_vec2();

    let scale = diff / zoom;

    let origin = 
        canvas_size.min.to_vec2()
        + diff * Vec2 { x: 0.5, y: 0.5 };

    let width_diff = scale * Vec2 { x: 1.0, y: 1.0 };

    let mut add_to_mesh = |x, y, col| {

        let x = origin.x + scale.x * x as f32;
        let y = origin.y + scale.y * y as f32;

        let x_max = x + width_diff.x;
        let y_max = y + width_diff.y;

        let min = Pos2{
            x,
            y
        };

        let max = Pos2{
            x: x_max,
            y: y_max
        };

        mesh.add_colored_rect(
            Rect{
                min, 
                max
            }, 
            col
        );
    };

    let total = 1.0 / (walker.history.len() as f32);

    let red_dist = (col1_grad.r() as i16 - color.r() as i16) as f32;
    let blue_dist = (col1_grad.b() as i16 - color.b() as i16) as f32;
    let green_dist = (col1_grad.g() as i16 - color.g() as i16) as f32;

    let r = color.r() as f32;
    let b = color.b() as f32;
    let g = color.g() as f32;


    for (pos, i) in walker.history.vec[old_total-1..].iter().zip(old_total..)
    {
        let p = i as f32 * total;
        let red: u8 = (r + p * red_dist) as u8;
        let green = (g + p * green_dist) as u8;
        let blue = (b + p * blue_dist) as u8;

        let col = Color32::from_rgb(red, green, blue);

        add_to_mesh(
            pos.x,
            pos.y,
            col
        );
    }

    add_to_mesh(
        walker.ort.x,
        walker.ort.y,
        color2
    );

}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PerformanceHint{
    PrioritizePerformance,
    PrioritizeOptics
}

#[derive(Clone, Debug)]
pub struct MeshChangeTracker{
    changed: bool,
    old_steps: usize,
    accumulated_change: usize
}

impl MeshChangeTracker{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self{
            changed: true,
            old_steps: 0,
            accumulated_change: 0
        }
    }

    pub fn reset(&mut self){
        self.changed = true;
        self.old_steps = 0;
    }

    pub fn request_redraw(&mut self)
    {
        self.changed = true;
    }

    pub fn redraw_finished(&mut self, steps: usize){
        self.changed = false;
        self.accumulated_change = 0;
        self.old_steps = steps;
    }

    pub fn check_if_needs_redraw(
        &self, 
        speed: f64,
        priority: PerformanceHint
    ) -> bool
    {
        if self.changed{
            true
        } else {
            match priority{
                PerformanceHint::PrioritizeOptics => {
                    self.accumulated_change > 0
                },
                _ => {
                    let speed = (speed * 18.0) as usize;
                    let threshold = speed.max(10);
                    self.accumulated_change > threshold
                }
            }
        }
    }

    pub fn new_steps(&mut self, steps: usize){
        let diff = steps - self.old_steps;
        self.old_steps = steps;
        self.accumulated_change += diff;
    }

    pub fn get_current_step(&self) -> usize {
        self.old_steps
    }
}