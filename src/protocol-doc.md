# Hangman Protocol
Evan Scherrer & Blythe Kelly  
CS 135 - Programming Languages

## Server-bound
Sent by client

### Request Game Update
Sent every 0.25 seconds automatically. Expected response: Game State packet  
Size: 16 bytes  
Byte 0: 0  
Bytes 2-15: unused
  
### Guess Letter
Sent in REPL to guess a letter. Expected response: None  
Size: 16 bytes  
Byte 0: 1
Byte 1: 0  
Byte 2: Char - letter to guess  
Bytes 3-15: unused  

### Guess Word
Sent from REPL to guess a word. Expected response: None  
Size: 16 bytes  
Byte 0: 1  
Byte 1: 1  
Bytes 2-15: Chars - letters of the guessed word  


## Client-bound
Sent by server  

### Game State
Sent in response to Request Game Update packet  
Size: 40 Bytes  
Bytes 0 - 13: bytes of the progress in the word  
Bytes 14 - 15: current Game Result:
* 0 0 : Lost  
* 0 1 : Won  
* 1 * : Running  

Byte 16: *number* of incorrect guesses (both letters and words)  
Bytes 17 - 40: bytes of already guessed letters (right & wrong)  

