pub mod game;

use game::{Game, GameErr, Token};
use std::io::Write;

fn main() {
    let width = 7;
    let height = 6;

    let mut game = Game::new(width, height);
    let mut token = Token::Green;

    println!("\n{}", game);

    let get_input = |token: Token| {
        let mut num: isize = -1;

        while num == -1 {
            print!("Play as {} at column: ", token);
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim_end().to_owned();

            match input.parse() {
                Ok(n) => num = n,
                Err(_) => println!("Invalid input {} (try between 0 and {})", input, width - 1),
            }
        }

        num as usize
    };

    loop {
        let x = get_input(token);

        match game.play(token, x) {
            Ok(state) => {
                println!("\n{}", game);
                match state {
                    game::GameState::Playing => token.toggle(),
                    game::GameState::Won(token) => break println!("{} won!", token),
                    game::GameState::Draw => break println!("It's a draw!"),
                }
            }
            Err(err) => match err {
                GameErr::InvalidState => println!("Game is on invalid state"),
                GameErr::InvalidCol => println!("Column {} doesn't exist!", x),
                GameErr::FullCol => println!("Column {} is full!", x),
            },
        }
    }
}
