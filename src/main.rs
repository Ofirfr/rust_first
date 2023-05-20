mod game;
use game::Game;
use game::CIRCLE;
use game::X;
fn main() {
    let mut game = Game::new();
    game.play(true, X);
}
