use eframe::epaint::*;

use crate::random_walker::RandomWalker;

pub fn calc_mesh(
    walker: &RandomWalker,
    canvas_size: Rect,
    zoom: f32
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

    for pos in walker.history.hash.iter()
    {
        add_to_mesh(
            pos.x,
            pos.y,
            Color32::BLUE
        );
    }

    add_to_mesh(
        walker.ort.x,
        walker.ort.y,
        Color32::RED
    );

    mesh

}