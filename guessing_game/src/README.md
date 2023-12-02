## This is the readme for the guessing game project
This is a guessing game which will start by asking the operator to guess a number between 1 and 100. Once the operator inputs a number, the program will compare the value input to the secret number and respond with either a message stating it is too low or too high and to try again. If at any point the correct secret number was guessed, the response will say, You win!

# How to run the game
```
git clone https://github.com/Quontron/rust.git
cd ~\guessing_game\
cargo run
```

# PlantUML diagram
I also incorporated a plantUML diagram to depict the full flow of the sequence diagram. Please see guessing_game.puml within the ~\guessing_game\src\guessing_game.puml