mod game;

use game::GameState;
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

const IP_ADDRESS: &str = "127.0.0.1:25565";

fn main() {
    // prompt user for a word
    let word = prompt_for_word();

    // initialize the game with the user's word
    let mut game: GameState = initialize_game(&word as &str);
    println!("Game initialized with word {}", &word);

    // start listening for data from the clients
    let listener = TcpListener::bind(IP_ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle individual packets & modify the GameState
        handle_connection(stream, &mut game);
    }
}

// prompt the user for a word to play the game with
fn prompt_for_word() -> String {
    println!("Please input a word, 2-14 ASCII characters");
    let mut word = String::new();

    loop {
        // empty out word buffer on each iteration of the loop to clear out previous input attempt
        word.clear();

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line"); // this can panic; add actual error handling?

        // trim whitespace and convert to uppercase
        word = word.trim().to_string().to_ascii_uppercase();

        if word.len() < 2 || word.len() > 14 {
            println!("Word must be between 2 and 14 characters long");
            continue; // restart loop to prompt for input again
        }

        if !word.chars().all(|c| c.is_ascii()) {
            println!("Word must contain only ASCII characters");
            continue; // restart loop to prompt for input again
        }

        // all checks passed, exit loop
        break;
    }

    word
}

// initialize game with given word
fn initialize_game(word: &str) -> GameState {
    let game = GameState::new(&word);
    game
}

// process incoming packets
fn handle_connection(mut stream: TcpStream, game: &mut GameState) {
    // create an empty buffer to read data into
    let mut guess_buffer = [0; 16];

    // read 16 bytes of data into the buffer
    let _ = stream.read(&mut guess_buffer);
    println!("read data from client {:?}", guess_buffer);

    match guess_buffer[0..2] {
        // match per protocol-doc
        [0, 0] => {
            // request game update
            // reply GameState to client
            let to_write = &game.serialize();
            // println!("{:?}", &to_write);
            let _ = stream.write(to_write);
        }
        [1, 0] => {
            // letter guess
            let ltr = guess_buffer[2] as char;
            game.guess_letter(ltr);
        }
        [1, 1] => {
            // word guess
            let bytes: Vec<u8> = guess_buffer
                .iter()
                .skip(2)
                .copied()
                .filter(|&b| b != 0)
                .collect();

            let wrd = String::from_utf8_lossy(&bytes);

            game.guess_word(&wrd);
        }
        _ => {
            println!("invalid packet {:?}", guess_buffer)
        }
    };
}
