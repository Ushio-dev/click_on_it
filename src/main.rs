mod square;

use ggez::{
    event::{self, MouseButton},
    graphics::{self, TextLayout},
    Context, GameResult,
};
use square::Square;

enum GameState {
    Playing,
    GameOver,
}
struct MainState {
    squares: Vec<Square>,
    speed: f32,
    state: GameState,
    score: i32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut objects = Vec::new();
        let an_square = Square::new(_ctx)?;
        objects.push(an_square);

        let s = MainState {
            squares: objects,
            speed: 0.5,
            state: GameState::Playing,
            score: 0,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        match self.state {
            GameState::Playing => {
                for _sqr in &mut self.squares {
                    _sqr.pos_y = _sqr.pos_y + self.speed;
                    _sqr.rect.x = _sqr.pos_x;
                    _sqr.rect.y = _sqr.pos_y;

                    if _sqr.pos_y >= 400. {
                        self.state = GameState::GameOver;
                    }
                }
            }
            GameState::GameOver => {
                self.squares.clear();
            }
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(_ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let text_score = format!("Score: {}", self.score);
        match self.state {
            GameState::Playing => {
                canvas.draw(graphics::Text::new(text_score).set_scale(20.).set_layout(
                    TextLayout {
                        h_align: graphics::TextAlign::Begin,
                        v_align: graphics::TextAlign::Begin,
                    }),
                    graphics::DrawParam::default().dest([5.,5.]),
                );
                for _sqr in &self.squares {
                    _sqr.draw(&mut canvas);
                }
            }
            GameState::GameOver => {
                canvas.draw(
                    graphics::Text::new("Game Over")
                        .set_scale(48.)
                        .set_layout(TextLayout {
                            h_align: graphics::TextAlign::Middle,
                            v_align: graphics::TextAlign::Middle,
                        }),
                    graphics::DrawParam::default().dest([640. / 2., 400. / 2.]),
                );
                canvas.draw(graphics::Text::new(text_score).set_scale(30.).set_layout(
                    TextLayout {
                        h_align: graphics::TextAlign::Middle,
                        v_align: graphics::TextAlign::Middle,
                    }),
                    graphics::DrawParam::default().dest([640. / 2., (400. / 2.)+ 50.]),
                );
            }
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
        let mut to_add = false;
        let mut index = 0;

        for (_i, _sqr) in self.squares.iter().enumerate() {
            if _button == MouseButton::Left {
                let point = _sqr.rect.contains([_x, _y]);
                if point {
                    index = _i;
                    to_add = true;
                    break;
                }
            }
        }

        if to_add {
            self.speed = self.speed + 0.03;
            self.score += 5;
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
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Click On It")
                .vsync(true),
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(640., 400.)
                .fullscreen_type(ggez::conf::FullscreenType::Windowed)
                .max_dimensions(640., 400.),
        );
    let (mut ctx, event_loop) = context_builder.build()?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
