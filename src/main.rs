use tetra::{
    graphics::{self, *},
    input::*,
    math::{self, vec2, FrustumPlanes, Vec2},
    window, Context, ContextBuilder, State,
};

fn main() -> tetra::Result {
    ContextBuilder::new("Hello", 900, 800)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    player: Player
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player = Player::new();
        Ok(GameState {player})
    }
}

struct Player {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}


impl Player {
    fn new() -> Player {
        let position = Vec2::new(0., 0.);
        let velocity = Vec2::new(0., 0.);
        Player {position: position, velocity: velocity}
    }
}