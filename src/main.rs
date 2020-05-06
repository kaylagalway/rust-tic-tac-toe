use std::io;

fn main() {
    // Create board array of numbers for printing board
    let mut board_array: [String; 9] = [
        String::from("0"), 
        String::from("1"), 
        String::from("2"), 
        String::from("3"), 
        String::from("4"), 
        String::from("5"), 
        String::from("6"), 
        String::from("7"), 
        String::from("8")
    ];

    println!("Welcome to the tic tac toe!");

    let mut is_player_one = true;
    let mut player_character = String::new();
    let mut turn_count = 1;
    let mut player_number = String::new();

    loop {
        // Check which player is currently going and adjust their character accordingly
        if is_player_one {
            player_number = String::from("1");
            player_character = String::from("X");
        } else {
            player_number = String::from("2");
            player_character = String::from("O");
        }

        // Print player prompts and board
        println!("Player {}, please enter a number, 0 to 8, to pick a spot for your play", player_number);
        print_board(&board_array);

        let mut chosen_index = String::new();

        // Prompt user for index input
        io::stdin().read_line(&mut chosen_index)
            .expect("Failed to read line");

        // Convert user input to usize to access board array
        let chosen_index: usize = match chosen_index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Need to add error check for if the number is between 0 and 9

        // Replace number in array with player character, X or O
        board_array[chosen_index] = player_character;
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

fn print_board(board_array: &[String]) {
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

fn check_for_win(board_array: &[String]) -> bool {
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


