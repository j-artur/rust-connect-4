use std::fmt::Display;

use multizip::zip4;

#[derive(Clone, Copy)]
pub enum GameState {
    Playing,
    Won(Token),
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Red,
    Green,
}

impl Token {
    pub fn toggle(&mut self) {
        match &self {
            Token::Red => *self = Token::Green,
            Token::Green => *self = Token::Red,
        }
    }
}

pub enum GameErr {
    InvalidState,
    InvalidCol,
    FullCol,
}

pub struct Game {
    state: GameState,
    width: usize,
    height: usize,
    grid: Vec<Vec<Token>>,
}

fn check_4(a: &Token, b: &Token, c: &Token, d: &Token) -> Option<Token> {
    (a == b && b == c && c == d).then(|| a.to_owned())
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            state: GameState::Playing,
            width,
            height,
            grid: vec![vec![]; width],
        }
    }

    pub fn play(&mut self, token: Token, x: usize) -> Result<GameState, GameErr> {
        match self.state {
            GameState::Playing => match self.grid.get_mut(x) {
                Some(vec) if vec.len() < self.height => {
                    let y = vec.len();
                    vec.push(token);

                    if self.grid.iter().any(|vec| vec.len() < self.height) {
                        self.check_win(x, y)
                    } else {
                        Ok(GameState::Draw)
                    }
                }
                Some(_) => Err(GameErr::FullCol),
                None => Err(GameErr::InvalidCol),
            },
            _ => Err(GameErr::InvalidState),
        }
    }

    fn check_win(&self, x: usize, y: usize) -> Result<GameState, GameErr> {
        match self.win_ver(x).or(self.win_hor(y)).or(self.win_dia(x, y)) {
            Some(token) => Ok(GameState::Won(token)),
            None => Ok(GameState::Playing),
        }
    }

    fn win_ver(&self, x: usize) -> Option<Token> {
        self.grid.get(x).and_then(|col| match col.len() {
            n if n < 4 => None,
            n => check_4(
                col.get(n - 1)?,
                col.get(n - 2)?,
                col.get(n - 3)?,
                col.get(n - 4)?,
            ),
        })
    }

    fn win_hor(&self, y: usize) -> Option<Token> {
        let it = || self.grid.iter();

        zip4(it(), it().skip(1), it().skip(2), it().skip(3))
            .find_map(|(c0, c1, c2, c3)| check_4(c0.get(y)?, c1.get(y)?, c2.get(y)?, c3.get(y)?))
    }

    fn win_dia(&self, x: usize, y: usize) -> Option<Token> {
        [-3, -2, -1, 0].iter().find_map(|i| {
            let y = (y as isize + i.to_owned()) as usize;

            let left_to_right = || {
                let x = (x as isize + *i) as usize;

                check_4(
                    self.grid.get(x)?.get(y)?,
                    self.grid.get(x + 1)?.get(y + 1)?,
                    self.grid.get(x + 2)?.get(y + 2)?,
                    self.grid.get(x + 3)?.get(y + 3)?,
                )
            };

            let right_to_left = || {
                let x = (x as isize - *i) as usize;

                check_4(
                    self.grid.get(x)?.get(y)?,
                    self.grid.get(x - 1)?.get(y + 1)?,
                    self.grid.get(x - 2)?.get(y + 2)?,
                    self.grid.get(x - 3)?.get(y + 3)?,
                )
            };

            left_to_right().or(right_to_left())
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "{}",
            match &self {
                Token::Red => "🔴",
                Token::Green => "🟢",
            }
        )?)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for i in (0..self.height).rev() {
            for j in 0..self.width {
                let dot = self
                    .grid
                    .get(j)
                    .and_then(|col| col.get(i))
                    .map_or("⚪".to_string(), |token| token.to_string());
                write!(f, "{}", dot)?;
            }
            write!(f, "\n")?
        })
    }
}
