use std::env::args;
use std::fmt;
use std::io;
use std::io::Write;
use std::iter;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    X,
    O,
    N,
}

impl State {
    fn players() -> &'static [State] {
        &[State::O, State::X]
    }
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
                'N' | 'n' | '_' => board.0.push(State::N),
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
        Board(iter::repeat(State::N).take(size * size).collect())
    }

    fn dimension(&self) -> usize {
        (self.0.len() as f64).sqrt() as usize
    }

    fn to_string_short(&self) -> String {
        let mut s = String::with_capacity(self.0.len() + self.dimension());
        for y in 0..self.dimension() {
            for x in 0..self.dimension() {
                s.push_str(&self[(x, y)].to_string())
            }
        }
        s
    }

    fn winner_with_criteria(&self, criteria: usize) -> Option<State> {
        for s in State::players() {
            let update = |coords: (usize, usize), counter: &mut usize| {
                if self[coords] == *s {
                    *counter += 1;
                } else {
                    *counter = 0;
                }
            };
            let winner = |candidates: &[usize]| -> bool {
                candidates.iter().find(|c| *c >= &criteria).is_some()
            };
            let (mut diar, mut dial) = (0usize, 0usize);
            for i in 0..self.dimension() {
                let (mut row, mut col) = (0usize, 0usize);
                for j in 0..self.dimension() {
                    update((i, j), &mut row);
                    update((j, i), &mut col);
                    if winner(&[row, col]) {
                        return Some(*s);
                    }
                }
                update((i, i), &mut dial);
                update((self.dimension() - i - 1, i), &mut diar);
                if winner(&[diar, dial]) {
                    return Some(*s);
                }
            }
        }
        None.or(Some(State::N).filter(|_| self.0.iter().filter(|s| *s == &State::N).count() == 0))
    }

    #[allow(unused)]
    fn winner(&self) -> Option<State> {
        self.winner_with_criteria(self.dimension())
    }
}

fn main() {
    let (mut board, criteria) = {
        let mut args = args().skip(1);
        let (size, criteria) = (
            args.next(),
            args.next().and_then(|c| c.parse::<usize>().ok()),
        );
        let board = if let Some(arg) = size {
            if let Ok(size) = arg.parse::<usize>() {
                Board::new(size)
            } else if let Ok(board) = Board::from_str(&arg) {
                board
            } else {
                Board::default()
            }
        } else {
            Board::default()
        };
        let dim = board.dimension();
        (board, criteria.unwrap_or(dim))
    };
    let players_rev = State::players().iter().rev().cloned().collect::<Vec<_>>();

    let players: &[State] = if SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() % 2 == 0)
        .unwrap_or(false)
    {
        State::players()
    } else {
        &players_rev[..]
    };
    let stdin = std::io::stdin();
    println!("{}", &board);
    let winner = loop {
        if let Some(winner) = board.winner_with_criteria(criteria) {
            break winner;
        }
        let mut input = String::new();
        for s in players {
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
            if let Some(_) = board.winner_with_criteria(criteria) {
                break;
            }
            println!("{}", &board);
        }
    };
    println!("{}\nThe winner is {}", board.to_string_short(), winner);
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
        assert_eq!(Board::from_str("XNX,NNN,NNN").unwrap().winner(), None);
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
        println!("{}", Board::from_str("XOX,OOX,OXO").unwrap());
        assert_eq!(
            Board::from_str("XOX,OOX,OXO").unwrap().winner(),
            Some(State::N)
        );
        assert_eq!(
            Board::from_str("XOX,OOX,OXO")
                .unwrap()
                .winner_with_criteria(1),
            Some(State::O)
        );

        assert_eq!(
            Board::from_str("XOX,OOX,OXO")
                .unwrap()
                .winner_with_criteria(2),
            Some(State::O)
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            &Board::from_str("XXX,NNN,NNN").unwrap().to_string_short()[..],
            "XXX______"
        );
    }
}
