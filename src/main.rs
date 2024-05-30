
use tetra::*;

fn main() -> tetra::Result {
    ContextBuilder::new("Hello", 900, 800)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {}

impl State for GameState {}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {})
    }
}


struct Player {
    position: Vec2<f32>,
    velocity: Vec2<f32>
}


