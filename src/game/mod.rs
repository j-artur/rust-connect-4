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

fn min_len_4<T>(a: &Vec<T>, b: &Vec<T>, c: &Vec<T>, d: &Vec<T>) -> usize {
    a.len().min(b.len()).min(c.len()).min(d.len())
}

fn check_4(a: &Token, b: &Token, c: &Token, d: &Token) -> Option<Token> {
    (a == b && b == c && c == d).then(|| a.to_owned())
}

fn check_4_option(
    a: Option<&Token>,
    b: Option<&Token>,
    c: Option<&Token>,
    d: Option<&Token>,
) -> Option<Token> {
    match (a, b, c, d) {
        (Some(a), Some(b), Some(c), Some(d)) => check_4(a, b, c, d),
        _ => None,
    }
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

    fn check_vertical(&self) -> Option<Token> {
        self.grid
            .iter()
            .filter(|col| col.len() >= 4)
            .find_map(|col| {
                let it = || col.iter();

                zip4(it(), it().skip(1), it().skip(2), it().skip(3))
                    .find_map(|(t0, t1, t2, t3)| check_4(t0, t1, t2, t3))
            })
    }

    fn check_horizontal(&self) -> Option<Token> {
        let it = || self.grid.iter();

        zip4(it(), it().skip(1), it().skip(2), it().skip(3)).find_map(|(c0, c1, c2, c3)| {
            (0..min_len_4(c0, c1, c2, c3))
                .find_map(|i| check_4_option(c0.get(i), c1.get(i), c2.get(i), c3.get(i)))
        })
    }

    fn check_diagonal(&self) -> Option<Token> {
        let it = || self.grid.iter();

        zip4(it(), it().skip(1), it().skip(2), it().skip(3)).find_map(|(c0, c1, c2, c3)| {
            let left_to_right = c0.len() >= 1 && c1.len() >= 2 && c2.len() >= 3 && c3.len() >= 4;
            let right_to_left = c0.len() >= 4 && c1.len() >= 3 && c2.len() >= 2 && c3.len() >= 1;

            let len = min_len_4(c0, c1, c2, c3);

            (0..len).find_map(|i| match (left_to_right, right_to_left) {
                (true, _) => check_4_option(c0.get(i), c1.get(i + 1), c2.get(i + 2), c3.get(i + 3)),
                (_, true) => check_4_option(c0.get(i + 3), c1.get(i + 2), c2.get(i + 1), c3.get(i)),
                _ => None,
            })
        })
    }

    pub fn play(&mut self, token: Token, col: usize) -> Result<GameState, GameErr> {
        match self.state {
            GameState::Playing => match self.grid.get_mut(col) {
                Some(vec) if vec.len() < self.height => {
                    vec.push(token);

                    if self.grid.iter().any(|vec| vec.len() < self.height) {
                        match self
                            .check_vertical()
                            .or(self.check_horizontal())
                            .or(self.check_diagonal())
                        {
                            Some(token) => Ok(GameState::Won(token)),
                            None => Ok(GameState::Playing),
                        }
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
                write!(f, "{dot}")?;
            }
            write!(f, "\n")?
        })
    }
}
