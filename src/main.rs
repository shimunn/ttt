use std::fmt;
use std::io;
use std::io::Write;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    X,
    O,
    N,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::X => 'X',
                State::O => 'O',
                State::N => '_',
            }
        )
    }
}

struct Board(Vec<State>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dim = self.dimension();
        for y in 0..dim {
            for x in 0..dim {
                write!(f, "[{}]", self[(x, y)])?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new(3)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = State;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.0[idx.0 + idx.1 * self.dimension()]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let dim = self.dimension();
        &mut self.0[idx.0 + idx.1 * dim]
    }
}

impl Board {
    fn new(size: usize) -> Board {
        let mut b = Vec::with_capacity(size * size);
        for _ in 0..b.capacity() {
            b.push(State::N);
        }
        Board(b)
    }

    fn dimension(&self) -> usize {
        (self.0.len() as f64).sqrt() as usize
    }

    fn winner(&self) -> Option<State> {
        let mut winners = [State::N; 4]; //winner row, col, diag right, diag left
        let dim = self.dimension();
        for a in 0..dim {
            for b in 0..dim {
                //winner row
                winners[0] = if b == 0 || (winners[0] != State::N && winners[0] == self[(a, b)]) {
                    self[(a, b)]
                } else {
                    State::N
                };
                //winner col
                winners[1] = if b == 0 || (winners[1] != State::N && winners[1] == self[(b, a)]) {
                    self[(b, a)]
                } else {
                    State::N
                };
                //winner diag
                if a == b {
                    winners[2] = if a == 0 || winners[2] != State::N && winners[2] == self[(a, a)] {
                        self[(a, a)]
                    } else {
                        State::N
                    };

                    winners[3] = if a == 0
                        || winners[3] != State::N && winners[3] == self[(dim - a - 1, dim - a - 1)]
                    {
                        self[(dim - a - 1, dim - a - 1)]
                    } else {
                        State::N
                    };
                }
            }
        }
        winners
            .into_iter()
            .find(|w| *w != &State::N)
            .map(|w| w.clone())
    }
}

fn main() {
    let mut board = Board::default();
    let mut stdin = std::io::stdin();
    let winner = loop {
        if let Some(winner) = board.winner() {
            break winner;
        }
        let mut input = String::new();
        for s in &[State::X, State::O] {
            loop {
                let (x, y) = loop {
                    print!("{}, your move: (x  y)  ", s);
                    io::stdout().flush();
                    input.clear();
                    stdin.read_line(&mut input);
                    let parts = input.trim().split(" ").collect::<Vec<_>>();
                    match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                        (Err(_), _) => eprintln!("X is not an valid int"),
                        (_, Err(_)) => eprintln!("Y is not an valid int"),
                        (Ok(x), Ok(y))
                            if (x > board.dimension()
                                || x < 1
                                || y > board.dimension()
                                || y < 1) =>
                        {
                            eprintln!("X or Y out of bounds")
                        }
                        (Ok(x), Ok(y)) => break (x - 1, y - 1),
                    }
                };
                match board[(x, y)] {
                    State::N => {
                        board[(x, y)] = *s;
                        break;
                    }
                    _ => eprintln!("({}, {}) is already occupied! Try again", x, y),
                }
            }
            if let Some(winner) = board.winner() {
                break;
            }
            println!("{}", &board);
        }
    };
    println!("The winner is {}", winner);
}
