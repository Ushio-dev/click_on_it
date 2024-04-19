use ggez::{glam::Vec2, graphics::{self, Color}, GameResult};
use rand::prelude::*;

pub struct Square {
    pub color: Color,
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
    pub mesh: ggez::graphics::Mesh,
    pub rect: ggez::graphics::Rect,
}

impl Square {
    pub fn new(_ctx: &mut ggez::Context) -> GameResult<Square> {
        let random = rand::thread_rng().gen_range(100..540) as f32;
        let random_color = rand::thread_rng().gen_range(1..=3);
        let color = match random_color {
            1 => ggez::graphics::Color::RED,

            2 => ggez::graphics::Color::GREEN,
            3 => ggez::graphics::Color::YELLOW,
            _ => ggez::graphics::Color::WHITE,
        };

        let rect = ggez::graphics::Rect::new(0., 0., 80., 80.);
        let mesh = ggez::graphics::Mesh::new_rectangle(
            _ctx, 
            ggez::graphics::DrawMode::fill(),
            rect, 
            color,
        )?;

        Ok(Square { 
            color: ggez::graphics::Color::RED,
            pos_x: random,
            pos_y: 0.,
            width: 70.,
            height: 70.,
            mesh,
            rect
        })
        
    }
    pub fn draw(&self, _canvas: &mut ggez::graphics::Canvas ) {
        //let color = [0.0, 0.0, 1.0, 1.0];
        _canvas.draw(&self.mesh, Vec2::new(self.pos_x, self.pos_y));
    }
}