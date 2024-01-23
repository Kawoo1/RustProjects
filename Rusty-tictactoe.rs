use std::io;
// The following code is authored by Kyle Shanahan
// In this Rust file is an executable version of tic tac toe. 
// The instructions to run the program are below:
// 1) Open a terminal and navigate to the directory where you want to create the project.
// 2) Use cargo new Rusty-tictactoe to create a new project.
// 3) Navigate to the Rusty-tictactoe with "cd Rusty-tictactoe".
// 4) Run the program with "cargo run".
// 5) The program will start in a new terminal window. Time to play!
// 6) To exit the program, press Ctrl + C.


#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: vec![Cell::Empty; 9],
        }
    }

    fn display(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            match cell {
                Cell::Empty => print!("{} ", i + 1),
                Cell::Occupied(Player::X) => print!("X "),
                Cell::Occupied(Player::O) => print!("O "),
            }

            if (i + 1) % 3 == 0 {
                println!();
            }
        }
        println!();
    }

    fn make_move(&mut self, position: usize, player: Player) -> Result<(), &'static str> {
        if position < 1 || position > 9 {
            return Err("Invalid move. Position must be between 1 and 9.");
        }

        let index = position - 1;
        if self.cells[index] != Cell::Empty {
            return Err("Invalid move. Cell already occupied.");
        }

        self.cells[index] = Cell::Occupied(player);
        Ok(())
    }
    
    // Check if there is a winner
    fn check_winner(&self) -> Option<Player> {
        // Check rows
        for i in (0..9).step_by(3) {
            if self.cells[i] == self.cells[i + 1] && self.cells[i] == self.cells[i + 2] {
                if let Cell::Occupied(player) = self.cells[i] {
                    return Some(player);
                }
            }
        }

        // Check columns
        for i in 0..3 {
            if self.cells[i] == self.cells[i + 3] && self.cells[i] == self.cells[i + 6] {
                if let Cell::Occupied(player) = self.cells[i] {
                    return Some(player);
                }
            }
        }

        // Check diagonals
        if (self.cells[0] == self.cells[4] && self.cells[0] == self.cells[8])
            || (self.cells[2] == self.cells[4] && self.cells[2] == self.cells[6])
        {
            if let Cell::Occupied(player) = self.cells[4] {
                return Some(player);
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|cell| *cell != Cell::Empty)
    }
}


fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        board.display();

        println!("Player {:?}, enter your move (1-9):", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let position: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match board.make_move(position, current_player) {
            Ok(_) => {
                if let Some(winner) = board.check_winner() {
                    board.display();
                    println!("Player {:?} wins!", winner);
                    break;
                } else if board.is_full() {
                    board.display();
                    println!("It's a draw!");
                    break;
                }

                current_player = match current_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
            }
            Err(err) => println!("{}", err),
        }
    }
}
