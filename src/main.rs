mod square;

use ggez::{event, glam::Vec2, graphics::{self}, GameResult};
use square::Square;

struct MainState {
    squares: Vec<Square>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut objects = Vec::new();
        let an_square = Square {
            color: ggez::graphics::Color::RED,
            height: 100.,
            width: 100.,
            pos_x: 200.,
            pos_y: 200.
        };
        objects.push(an_square);
        
        let s = MainState {
            squares: objects
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())    
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for _sqr in &self.squares {
            let square = graphics::Mesh::new_rectangle(
                _ctx, 
                graphics::DrawMode::fill(), 
                ggez::graphics::Rect::new(_sqr.pos_x,_sqr.pos_y, _sqr.width, _sqr.height), 
                _sqr.color
            )?;
            canvas.draw(&square, Vec2::new(100.,100.));
        }
    
        canvas.finish(_ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), ggez::GameError> {
        let new_square = Square {
            pos_x: 50.,
            pos_y: 50.,
            color: ggez::graphics::Color::YELLOW,
            width: 100.,
            height: 100.
        };
        self.squares.push(new_square);
        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("nose", "Franco");
    let (ctx, event_loop) = context_builder.build()?;
    let state = MainState::new()?;

    event::run(ctx, event_loop, state)
}
