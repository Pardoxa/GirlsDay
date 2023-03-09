use eframe::epaint::*;

use crate::random_walker::RandomWalker;

pub fn calc_mesh(
    walker: &RandomWalker,
    canvas_size: Rect,
    zoom: u16
) -> Mesh
{
    let mut mesh = Mesh::default();

    let diff = canvas_size.max.to_vec2()
        - canvas_size.min.to_vec2();

    let scale = diff / (zoom as f32);

    let origin = 
        canvas_size.min.to_vec2()
        + diff * Vec2 { x: 0.5, y: 0.5 };

    let mut add_to_mesh = |x, y, col| {

        let min = origin 
            +scale * Vec2 { x: x as f32, y: y as f32 };
        
        let max = origin 
            +scale * Vec2 { x: (x + 1) as f32, y: (y + 1) as f32 };

        mesh.add_colored_rect(
            Rect{
                min: min.to_pos2(), 
                max: max.to_pos2()
            }, 
            col
        );
    };

    for pos in walker.history.iter()
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