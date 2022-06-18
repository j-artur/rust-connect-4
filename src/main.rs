pub mod game;

use game::{Game, GameErr, Token};
use std::io::Write;

fn main() {
    let width = 7;
    let height = 6;

    let mut game = Game::new(width, height);
    let mut token = Token::Green;

    println!("\n{game}");

    let get_input = |token: Token| {
        let mut num: isize = -1;

        while num == -1 {
            print!("Play as {token} at column: ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim_end().to_owned();

            match input.parse() {
                Ok(n) => num = n,
                Err(_) => println!("Invalid input {input} (try between 0 and {})", width - 1),
            }
        }

        num as usize
    };

    loop {
        let col = get_input(token);

        match game.play(token, col) {
            Ok(state) => match state {
                game::GameState::Playing => {
                    println!("\n{game}");
                    token.toggle();
                }
                game::GameState::Won(token) => {
                    println!("\n{game}");
                    break println!("{token} won!");
                }
                game::GameState::Draw => {
                    println!("\n{game}");
                    break println!("It's a draw!");
                }
            },
            Err(err) => match err {
                GameErr::InvalidState => println!("Game is on invalid state"),
                GameErr::InvalidCol => println!("Column {col} doesn't exist!"),
                GameErr::FullCol => println!("Column {col} is full!"),
            },
        }
    }
}
