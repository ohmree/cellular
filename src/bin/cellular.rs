use cellular::automata::gol::*;
use cellular::Automaton;

pub fn main() {
    let game = GameOfLife::new((10, 10));
    // This is just dummy code to get this to compile
    game.print();
    let step2 = game.step();
    step2.print();
}
