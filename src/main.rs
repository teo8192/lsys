use std::error::Error;

use draw::*;

mod graphics;
mod lsystem;

use graphics::{Graphics, TurtleConfig};
use lsystem::LSystem;

const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 300.0;

impl Graphics<()> for Canvas {
    fn draw_line(&mut self, c_0: (f32, f32), c_1: (f32, f32)) -> Result<(), ()> {
        let x_off = WIDTH / 2.0;
        let y_off = HEIGHT / 2.0;

        let line = Drawing::new()
            .with_shape(Shape::Line {
                start: Point::new(c_0.0 + x_off, c_0.1 + y_off),
                points: vec![shape::LinePoint::Straight {
                    point: Point::new(c_1.0 + x_off, c_1.1 + y_off),
                }],
            })
            .with_style(Style::stroked(1, Color::black()));
        self.display_list.add(line);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lsys = LSystem::from_str("++++F F->G[+F][-F]-GF G->GG")?;

    let iters = 9;

    let word = lsys.nth(iters).unwrap();

    let turtle = TurtleConfig::default()
        .stepsize(75.0 * (2f32.powf(-(iters as f32))))
        .delta_ang(std::f32::consts::PI / 6.0)
        .draw_forward("FG");

    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);

    turtle.create_turtle().draw(&mut canvas, word).unwrap();

    render::save(&canvas, "thing.svg", SvgRenderer::new()).expect("Failed to save");

    Ok(())
}
