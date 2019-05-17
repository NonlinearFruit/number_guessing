# Number Guessing

[![build](https://travis-ci.org/NonlinearFruit/number_guessing.svg?branch=master)](https://travis-ci.org/NonlinearFruit/number_guessing)

Can you guess my random number? Set the upper bound. You have 8 guesses. Your upper bound is your score (if you succeed).

## Description

The game will generate a random integer between 1 and {upper_bound}, inclusive. It will then prompt the player to enter a guess, indicating if the guess is too high or too low. If the guess is correct, the game will print a congratulatory message and compare the player's upper_bound to the current high score. If the upper_bound surpasses the high score, the game congratulates the player and update the current high score on file.

## Background

A custom variant of the number guessing game in [chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) of _[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)_ book.
