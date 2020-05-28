use std::io;

struct Board {
    array: [&'static str; 9],
}

impl Board {
    fn print_board(&self) {
        // Board should print as:
        // |0|1|2|
        // |3|4|5|
        // |6|7|8|
        println!("
        |{}|{}|{}|
        |{}|{}|{}|
        |{}|{}|{}|",
        self.array[0],
        self.array[1],
        self.array[2],
        self.array[3],
        self.array[4],
        self.array[5],
        self.array[6],
        self.array[7],
        self.array[8]
        );
    }

    // Compare characters by indexes on board to verify if there is currently a winner
    fn check_for_win(&self) -> bool {
        if self.array[0] == self.array[1] && self.array[1] == self.array[2] {
            return true;
        } else if self.array[3] == self.array[4] && self.array[4] == self.array[5] {
            return true;
        } else if self.array[6] == self.array[7] && self.array[7] == self.array[8] {
            return true;
        } else if self.array[0] == self.array[3] && self.array[3] == self.array[6] {
            return true;
        } else if self.array[1] == self.array[4] && self.array[4] == self.array[7] {
            return true;
        } else if self.array[2] == self.array[5] && self.array[5] == self.array[8] {
            return true;
        } else if self.array[0] == self.array[4] && self.array[4] == self.array[8] {
            return true;
        } else if self.array[2] == self.array[4] && self.array[4] == self.array[6] {
            return true;
        } else {
            return false;
        }
    }

    // Prompts user to enter board position and handles user input
    fn get_player_entry(&self, player_number: &str) -> Result<usize, EntryError> {
        // Print player prompts and board
        println!(
        "Player {}, please enter a number, 0 to 8, to pick a spot for your play. Enter q to quit game.",
        player_number
    );
        self.print_board();

        let mut chosen_index = String::new();

        // Prompt user for index input
        io::stdin()
            .read_line(&mut chosen_index)
            .expect("Failed to read line");

        // Convert user input to usize to access board array
        match chosen_index.trim().parse() {
            Ok(num) if num <= 8 => Ok(num),
            // User entered number greater than 8
            Ok(_) => Err(EntryError::InvalidIndex),
            // User entered `q` character to quit game
            Err(_) if chosen_index.trim() == "q" => Err(EntryError::UserQuit),
            // User entered none int character
            Err(_) => Err(EntryError::InvalidCharacter),
        }
    }

    fn set_character(&mut self, index: usize, character: &'static str) {
        self.array[index] = character;
    }
}

fn main() {
    // Create board array of numbers for printing board
    let mut board: Board = setup_game();

    let mut is_player_one = true;
    let mut player_character;
    let mut turn_count = 1;
    let mut player_number;

    loop {
        // Set player data for this current round, player number and character
        let info = player_information(is_player_one);
        player_number = info.0;
        player_character = info.1;

        // Create optional player entry, value to be set by user input
        let mut player_entry: Option<usize> = None;

        loop {
            // Get user input to set player board entry
            // break loop if user chooses to quit and don't set value for player entry
            player_entry = match board.get_player_entry(&player_number) {
                Ok(index) => Some(index),
                Err(error) => {
                    match error {
                        EntryError::UserQuit => {
                            println!("You have chosen to end the game. See you next round!");
                            break;
                        }
                        EntryError::InvalidCharacter => println!(
                            "You did not enter a number. Please enter a number between 0 and 8."
                        ),
                        EntryError::InvalidIndex => println!(
                            "You did not enter a number between 0 and 8. Please try again."
                        ),
                    };
                    continue;
                }
            };
            break;
        }

        // Unwrap player_entry
        match player_entry {
            // Replace number in array with player character that user entered, X or O
            Some(value) => {
                board.set_character(value, player_character);
                board.print_board();
            }
            // if player_entry never initialized, this means user chose to quit
            // break the loop and end the game
            None => break,
        }

        // Check for winner
        let is_winner = board.check_for_win();

        // Get game state based on winner status and print win status for user
        let game_state = handle_win_result(is_winner, player_number, turn_count);

        // Handle game state, either end or continue turns
        match game_state {
            GameState::End => break,
            GameState::Continue => {
                is_player_one = !is_player_one;
                turn_count = turn_count + 1;
                continue;
            }
        }
    }
}

// Check which player is currently going and adjust their character accordingly
// Returns tuple of (player_number, player_character)
fn player_information(is_player_one: bool) -> (&'static str, &'static str) {
    if is_player_one {
        ("1", "X")
    } else {
        ("2", "O")
    }
}

// Create board array of numbers for printing board and print welcome message for user
fn setup_game() -> Board {
    println!("Welcome to the tic tac toe!");
    Board {
        array: ["0", "1", "2", "3", "4", "5", "6", "7", "8"],
    }
}

// Error returned if user enters invalid character when prompted for board index input
// InvalidIndex: Number greater than 8 entered, larger than board size
// InvalidCharacter: None usize convertible character entered
#[derive(Debug)]
enum EntryError {
    InvalidIndex,
    InvalidCharacter,
    UserQuit,
}

// Enum for state of game
// Continue: No current winner, run game loop again
// End: Winner found, end game
enum GameState {
    Continue,
    End,
}

// Checks game status, prints end game message if necessary, and returns game state
fn handle_win_result(is_winner: bool, player_number: &str, turn_count: i32) -> GameState {
    if is_winner {
        println!("Congratulations, you won player {}!", player_number);
        GameState::End
    } else if turn_count == 9 {
        println!("All positions on the board have been filled and there is no winner. Game over.");
        GameState::End
    } else {
        GameState::Continue
    }
}
