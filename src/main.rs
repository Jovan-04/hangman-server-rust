mod game;

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use game::GameState;

const IP_ADDRESS: &str = "127.0.0.1:25565";

fn main() {
    // TODO: prompt for word on startup - limit to 14 ASCII chars
    let word = "ballet"; // limit of 14 ASCII characters - enforce that
    let mut game: GameState = initialize_game(word);

    let listener = TcpListener::bind(IP_ADDRESS).unwrap();

    // individual "packet"
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established! {:?}", stream);
        handle_connection(stream, &mut game);
    }
}

// initialize game with given word
fn initialize_game(word: &str) -> GameState {
    let game = GameState::new(&word);
    game
}

fn handle_connection(mut stream: TcpStream, game: &mut GameState) {
    // create an empty buffer to read data into
    let mut guess_buffer = [0; 16];

    // read 16 bytes of data into the buffer
    let _ = stream.read(&mut guess_buffer);
    println!("read data from client {:?}", guess_buffer);

    match guess_buffer[0..2] { // match per protocol-doc
        [0, 0] => { // request game update
            // reply GameState to client
            let to_write = &game.serialize();
            // println!("{:?}", &to_write);
            let _ = stream.write(to_write);
        },
        [1, 0] => { // letter guess
            let ltr = guess_buffer[2] as char;
            game.guess_letter(ltr);
        },
        [1, 1] => { // word guess
            let bytes: Vec<u8> = guess_buffer.iter().skip(2).copied().filter(|&b| b != 0).collect();

            let wrd = String::from_utf8_lossy(&bytes);

            game.guess_word(&wrd);

        },
        _  => {
            println!("invalid packet {:?}", guess_buffer)
        },
    };

}