mod square;

use ggez::event;
use square::Square;

struct MainState {
    squares: Vec<Square>,
}

impl MainState {
    fn new() -> Self {
        let objects = Vec::new();
        Self { squares: objects }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())    
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }
}

fn main() {
    
}
