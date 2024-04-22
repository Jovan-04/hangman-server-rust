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

    // TODO: update game state based on buffer (see protocol-doc)

    // reply new GS to client
    let to_write = &game.serialize();
    // println!("{:?}", &to_write);
    let _ = stream.write(to_write);

}