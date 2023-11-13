use std::{thread, time};

const TICK_DURATION: time::Duration = time::Duration::from_millis(200);
const BOARD_SIZE: usize = 32;

pub struct Board {
    board: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.board[x - 1][y - 1] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        return if x == 0 || y == 0 || x > BOARD_SIZE || y > BOARD_SIZE {
            false
        } else {
            self.board[x - 1][y - 1]
        };
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] {
                    print!("x");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn next_generattion(&mut self) {
        let mut next_board = [[false; BOARD_SIZE]; BOARD_SIZE];
        for x in 1..=BOARD_SIZE {
            for y in 1..=BOARD_SIZE {
                let neighbors: [bool; 8] = [
                    self.get(x - 1, y - 1),
                    self.get(x - 1, y),
                    self.get(x - 1, y + 1),
                    self.get(x, y - 1),
                    self.get(x, y + 1),
                    self.get(x + 1, y - 1),
                    self.get(x + 1, y),
                    self.get(x + 1, y + 1),
                ];
                next_board[x - 1][y - 1] = next_state(self.get(x, y), neighbors)
            }
        }
        self.board = next_board;
    }
}

fn next_state(state: bool, neighbors: [bool; 8]) -> bool {
    // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    // Any live cell with two or three live neighbours lives on to the next generation.
    // Any live cell with more than three live neighbours dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
    let cnt: usize = neighbors.iter().filter(|x| **x).count();
    match (state, cnt) {
        (false, 3) => true,
        (true, _) if cnt > 3 => false,
        (true, _) if cnt < 2 => false,
        _ => state,
    }
}

fn main() {
    let mut b = Board::new();

    // fan
    b.set(2, 11, true);
    b.set(2, 12, true);
    b.set(2, 3, true);

    // glider
    b.set(4, 1, true);
    b.set(4, 2, true);
    b.set(4, 3, true);
    b.set(3, 3, true);
    b.set(2, 2, true);

    let mut gen = 0usize;

    while true {
        b.print();
        println!("Generation: {gen}");
        gen += 1;
        thread::sleep(TICK_DURATION);
        b.next_generattion();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_state() {
        assert_eq!(
            next_state(true, [true, true, true, true, true, true, true, true]),
            false
        );
        assert_eq!(
            next_state(
                true,
                [true, false, false, false, false, false, false, false]
            ),
            false
        );
        assert_eq!(
            next_state(true, [false, false, false, true, true, false, false, false]),
            true
        );
        assert_eq!(
            next_state(true, [false, false, false, true, true, false, true, false]),
            true
        );
        assert_eq!(
            next_state(false, [true, true, true, true, true, true, true, true]),
            false
        );
        assert_eq!(
            next_state(
                false,
                [false, false, false, true, true, false, false, false]
            ),
            false
        );
        assert_eq!(
            next_state(false, [false, false, false, true, true, false, true, false]),
            true
        );
    }

    #[test]
    fn test_get() {
        let mut b = Board::new();

        b.set(1, 1, true);
        assert_eq!(b.get(1, 1), true);

        assert_eq!(b.get(0, 1), false);
        assert_eq!(b.get(1, 0), false);
        assert_eq!(b.get(1, BOARD_SIZE), false);
        assert_eq!(b.get(BOARD_SIZE, 1), false);
    }
}
