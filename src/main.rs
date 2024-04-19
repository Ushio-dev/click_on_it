mod square;
mod colors;

use ggez::{
    event::{self, MouseButton},
    graphics::{self},
    Context, GameResult
};
use square::Square;

struct MainState {
    squares: Vec<Square>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut objects = Vec::new();
        let an_square = Square::new(_ctx)?;
        objects.push(an_square);

        let s = MainState { squares: objects };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        for _sqr in &mut self.squares {
            _sqr.pos_y = _sqr.pos_y + 1.;
            _sqr.rect.x = _sqr.pos_x;
            _sqr.rect.y = _sqr.pos_y;
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(_ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for _sqr in &self.squares {
            _sqr.draw(&mut canvas);
        }

        //println!("X: {}, Y: {}", self.squares[0].rect.point().x,self.squares[0].rect.point().y);
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
        let mut to_add = false;  
        let mut index = 0;
        
        for (_i, _sqr) in self.squares.iter().enumerate() {
            if _button == MouseButton::Left {
                //println!("Click!");
                let point = _sqr.rect.contains([_x, _y]);
                if point {
                    index = _i;
                    to_add = true;
                    break;
                }
            }
        }
        
        if to_add {
            let new_square: Square;
            new_square = Square::new(_ctx)?;
            self.squares.remove(index);
            self.squares.push(new_square);
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("nose", "Franco")
        .window_setup(ggez::conf::WindowSetup::default().title("Click On It"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(640., 400.));
    let (mut ctx, event_loop) = context_builder.build()?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
