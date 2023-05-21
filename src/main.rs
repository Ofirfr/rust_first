mod game;
use game::Game;
use game::X;
fn main() {
    let mut game = Game::new();
    game.play(true, X);
}
