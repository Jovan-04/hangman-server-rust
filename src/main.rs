mod game;

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use game::GameState;


fn main() {
    let mut game: GameState = initialize_game();

    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    // individual "packet"
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established! {:?}", stream);
        handle_connection(stream, &mut game);
    }
}

fn initialize_game() -> GameState {
    let word = "ballet"; // limit of 14 ASCII characters
    let game = GameState::new(&word);
    game
}


fn handle_connection(mut stream: TcpStream, game: &mut GameState) {
    let mut guess_buffer = [0; 16];

    let _ = stream.read(&mut guess_buffer);
    println!("read data from client {:?}", guess_buffer);
    // update game state

    // reply new GS to client
    let to_write = &game.serialize();
    // println!("{:?}", &to_write);
    let _ = stream.write(to_write);

    // let _ = stream.write(&[b'h', b'e', b'l', b'l', b'o', b'w', b'o', b'r', b'l', b'd']);

}