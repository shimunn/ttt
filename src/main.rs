use std::fmt;
use std::io;
use std::io::Write;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

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

#[derive(Clone, Debug, PartialEq)]
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

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Self::new(0);
        for c in s.chars() {
            match c {
                'X' | 'x' => board.0.push(State::X),
                'O' | 'o' => board.0.push(State::O),
                'N' | 'n' => board.0.push(State::N),
                _ => (),
            }
        }
        if board.dimension() * board.dimension() != board.0.len() {
            return Err(());
        }
        Ok(board)
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
        let mut winners = [State::N; 4];
        let dim = self.dimension();
        fn winner(winners: &[State]) -> Option<State> {
            winners
                .into_iter()
                .find(|w| *w != &State::N)
                .map(|w| w.clone())
        }
        for x in 0..dim {
            for y in 0..dim {
                let row = (y, x);
                let col = (x, y);

                {
                    let r = self[row];
                    let c = self[col];
                    winners[0] = if y == 0 || winners[0] == r {
                        r
                    } else {
                        State::N
                    };

                    winners[1] = if y == 0 || winners[1] == c {
                        c
                    } else {
                        State::N
                    };
                }
            }
            {
                //diagonal
                let diag_left = (x, x);
                let diag_right = (dim - x - 1, x);

                let l = self[diag_left];
                let r = self[diag_right];
                winners[2] = if x == 0 || winners[2] == l {
                    l
                } else {
                    State::N
                };

                winners[3] = if x == 0 || winners[3] == r {
                    r
                } else {
                    State::N
                };
            }
            //Row and Col winners can be determined after one Y pass
            if let Some(winner) = winner(&winners[0..2]) {
                return Some(winner);
            }
        }
        //Diagonal winners require a full X pass
        winner(&winners[2..4])
    }
}

fn main() {
    let mut board = Board::default();
    let stdin = std::io::stdin();
    println!("{}", &board);
    let winner = loop {
        if let Some(winner) = board.winner() {
            break winner;
        }
        let mut input = String::new();
        for s in &[State::X, State::O] {
            loop {
                let (x, y) = loop {
                    print!("{}, your move: (x  y)  ", s);
                    io::stdout().flush().unwrap();
                    input.clear();
                    stdin.read_line(&mut input).unwrap();
                    let parts = input.trim().split(" ").collect::<Vec<_>>();
                    if parts.len() != 2 {
                        eprintln!("Invalid input");
                        continue;
                    }
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
            if let Some(_) = board.winner() {
                break;
            }
            println!("{}", &board);
        }
    };
    println!("The winner is {}", winner);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn winner() {
        assert_eq!(Board::from_str("NNN,NNN,NNN").unwrap(), Board::default());
        assert_eq!(
            Board::from_str("XXX,NNN,NNN").unwrap().winner(),
            Some(State::X)
        );
        assert_eq!(
            Board::from_str("XNX,NNN,NNN").unwrap().winner(),
            None
        );
        assert_eq!(
            Board::from_str("NNN,XXX,NNN").unwrap().winner(),
            Some(State::X)
        );
        assert_eq!(
            Board::from_str("XON,OXN,ONX").unwrap().winner(),
            Some(State::X)
        );
        assert_eq!(
            Board::from_str("OXN,OON,XNO").unwrap().winner(),
            Some(State::O)
        );
    }
}
