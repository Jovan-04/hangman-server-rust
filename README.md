## Hangman Server
This is my first project in Rust, the server for a networked implementation of the popular whiteboard game Hangman. I wrote it (and the network protocol with it) at the end of my sophomore year of college as an add-on to the final project for my Programming Languages class. You can read more about the protocol specs in `protocol-doc.md`.  
**Note: This project requires my [hangman-client-rust](https://github.com/Jovan-04/hangman-client-rust) repository to work!** That is the client that allows you to actually play the game.  

### Setup
*tested on linux, but it should mostly work for other OSes too*  
1. Install Rust using the guide [here](https://www.rust-lang.org/tools/install)  
2. Open a terminal and navigate to an appropriate directory (such as your Downloads)  
3. Clone this repo with `git clone https://github.com/Jovan-04/hangman-server-rust.git`  
4. Run `cd hangman-server-rust` to navigate to the project's root directory  
5. By default, the server is hosted on port 25565; you can change that by editing the `IP_ADDRESS` string in `src/main.rs:9`  
* You can also build this to a binary executable for your OS using `cargo build`. The output file will be `./target/debug/server`.  

### Usage
1. Run the project's source code by navigating to its root directory and running `cargo run` in the terminal  
2. Enter a single word, 2-14 letters and case-insensitive when the terminal prompts you  
3. You'll get a message that reads `Game initialized with word {word}`, after which the game will begin logging all received packets to the terminal  
4. That's it! You can stop the game at any time with `Ctrl-C`  
