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