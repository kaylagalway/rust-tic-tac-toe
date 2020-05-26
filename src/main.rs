use std::io;

fn main() {
    // Create board array of numbers for printing board
    let mut board_array: [&'static str; 9] = [
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
    ];

    println!("Welcome to the tic tac toe!");

    let mut is_player_one = true;
    let mut player_character;
    let mut turn_count = 1;
    let mut player_number;

    loop {
        let info = player_information(is_player_one);
        player_number = info.0;
        player_character = info.1;
        
        let mut player_entry: usize;

        loop {
            player_entry = match get_player_entry(&player_number, &board_array) {
                Ok(index) => index,
                Err(error) => {
                    match error {
                        EntryError::InvalidCharacter => println!("You did not enter a number. Please enter a number between 0 and 8."),
                        EntryError::InvalidIndex => println!("You did not enter a number between 0 and 8. Please try again.")
                    };
                    continue;
                }
            };
            break;
        }

        // Replace number in array with player character, X or O
        board_array[player_entry] = player_character;
        print_board(&board_array);
        let is_winner = check_for_win(&board_array);
        if is_winner {
            println!("Congratulations, you won player {}!", player_number);
            break;
        } else if turn_count == 9 {
            println!("All positions on the board have been filled and there is no winner. Game over.");
            break;
        } else {
            is_player_one = !is_player_one;
            turn_count = turn_count + 1;
            continue;
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

fn setup_game() {

}

enum EntryError {
    InvalidIndex,
    InvalidCharacter
}

fn get_player_entry(player_number: &str, board_array: &[&str]) -> Result<usize, EntryError> {
    // Print player prompts and board
    println!("Player {}, please enter a number, 0 to 8, to pick a spot for your play", player_number);
    print_board(&board_array);

    let mut chosen_index = String::new();

    // Prompt user for index input
    io::stdin().read_line(&mut chosen_index)
        .expect("Failed to read line");

    // Convert user input to usize to access board array
    match chosen_index.trim().parse() {
        Ok(num) if num <= 8 => Ok(num),
        Ok(_) => Err(EntryError::InvalidIndex),
        Err(_) => Err(EntryError::InvalidCharacter),
    }
}

fn print_board(board_array: &[&str]) {
    // Board should print as:
    // |0|1|2|
    // |3|4|5|
    // |6|7|8|
    println!("
    |{}|{}|{}|
    |{}|{}|{}|
    |{}|{}|{}|", 
    board_array[0], board_array[1], board_array[2], board_array[3], board_array[4], board_array[5], board_array[6], board_array[7], board_array[8]);
}

fn check_for_win(board_array: &[&str]) -> bool {
    if board_array[0] == board_array[1] && board_array[1] == board_array[2] {
        return true;
    } else if board_array[3] == board_array[4] && board_array[4] == board_array[5] {
        return true;
    } else if board_array[6] == board_array[7] && board_array[7] == board_array[8] {
        return true;
    } else if board_array[0] == board_array[3] && board_array[3] == board_array[6] {
        return true;
    } else if board_array[1] == board_array[4] && board_array[4] == board_array[7] {
        return true;
    } else if board_array[2] == board_array[5] && board_array[5] == board_array[8] {
        return true;
    } else if board_array[0] == board_array[4] && board_array[4] == board_array[8] {
        return true;
    } else if board_array[2] == board_array[4] && board_array[4] == board_array[6] {
        return true;
    } else {
        return false
    }
}


