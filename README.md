# Rust experiments 
A collection of rust packages which taught me rust by messing around.

The collection currently contains three finished and one broken project:
- hello_rust
- minesweeper (broken)
- parse_roman_numerals
- simple_dfa

**Note that hello_rust and minesweeper are written in German, regarding the documentation and the console output.**


## Motivation

In one of my university courses someone meantioned rust and I decided to learn more.
On this journy I wrote many small conding projects to make mistakes and learn from them.
To collect these experiments for me and others to look at, I made this repository.

## History

### hello_rust

After deciding to learn rust my search quickly lead me to the [rust handbook](https://doc.rust-lang.org/book/).
At the end of chapter 3 it had some suggestions for easy programming challenges.
I implemented those together with the number guessing game of chapter 2.
To make it more compact I wrote the main function to be a kind of menu, from a sub program can be started.

### minesweeper

On a trip in the Deutsche Bahn (ICE to be specific) I got bored and wanted to program a simple game.
Because I had done tic tac toe before, I decided to make minesweeper.
Since I am not an artist I decided to display everything in the console.

**Right now the project does not compile, because I wanted to add some feature and got distracted.**

### parse_roman_numerals and simple_dfa

I got inspired by [a reddit post](https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/), combined with my love of roman numerals to write a program to return the decimal value of a given string of roman numerals.
After I got it to work, I noticed a lot of nonsense inputs that were possible, like `IMXDV`.
This happend after one very theoretical university course about automatons.
So I decided to find the regular expression for roman numerals (with subtraction rule) and implement a parser to accept or deny the input.
The course helped me a lot with the theoretical background, so I just had to think about translate automaton theory in rust code.

As a test I decided to write an automaton for the simplest (interesting) regex I could imagine `ab*a`.
I got this to work by using a total deterministic automaton, which I sketched out on paper and the implemented.
Armed with this knowledge I wrote the automaton for the roman numerals and it compiled and worked correctly on the first try.
