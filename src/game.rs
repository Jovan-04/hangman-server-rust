pub enum GameResult {
    Running,
    Won,
    Lost,
}

pub struct GameState {
    pub word: String,
    pub word_progress: Vec<char>,
    pub game_result: GameResult,
    pub incorrect_guesses: u8,
    pub letters_guessed: Vec<char>,
}

impl GameState {
    // create a new GameState object
    pub fn new(word: &str) -> Self {
        GameState {
            word: String::from(word),
            word_progress: vec!['_'; word.len()],
            game_result: GameResult::Running,
            incorrect_guesses: 0,
            letters_guessed: Vec::new(),
        }
    }

    // convert GameState object to a 40-byte array, according to protocol-doc
    pub fn serialize(&mut self) -> [u8; 40] {
        // initialize byte array of zeros
        let mut serialized = [0; 40];

        // replace part of byte array (max 14 bytes) with bytes in word_progress
        for (i, c) in self.word_progress.iter().enumerate() {
            serialized[i] = *c as u8;
        }

        // set bytes 14 & 15 per protocol-doc
        match self.game_result {
            GameResult::Lost => { serialized[14] = 0; serialized[15] = 0; },
            GameResult::Won => { serialized[14] = 0; serialized[15] = 1; },
            GameResult::Running => { serialized[14] = 1; serialized[15] = 0; },
        };

        // set byte 16 to the number of incorrect guesses
        serialized[16] = self.incorrect_guesses;

        // push rest of the bytes to the array
        for (i, c) in self.letters_guessed.iter().enumerate() {
            serialized[i+17] = *c as u8;
        }

        serialized
    }

    pub fn guess_letter(&mut self, guess: char) {
        if self.letters_guessed.contains(&guess) {
            return
        }

        let mut indices = Vec::new();
        for (i, char) in self.word.chars().enumerate() {
            if char == guess {
                indices.push(i);
            }
        }

        self.letters_guessed.push(guess);

        if indices.is_empty() {
            self.incorrect_guesses += 1;
        } else {
            for i in indices {
                self.word_progress[i] = guess;
            }
        }

        self.update_game_result();
    }

    pub fn guess_word(&mut self, guess: &str) {
        if guess == self.word {
            self.word_progress = Vec::from_iter(self.word.chars());
        } else {
            self.incorrect_guesses += 1;
        }

        self.update_game_result();
    }

    pub fn update_game_result(&mut self) {
        if self.incorrect_guesses >= 6 { // 6 incorrect guesses for a loss - make this a variable?
            self.game_result = GameResult::Lost;
        }

        if !self.word_progress.contains(&'_') {
            self.game_result = GameResult::Won;
        }
    }
}
