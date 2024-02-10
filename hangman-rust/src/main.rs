# ![allow(dead_code, unused_variables, unused_imports)]

use std::env::{args, Args};
use std::io::{self, stdin, stdout, Write};
use rand::Rng;

fn main() {
    let list_of_words = [("trump", "noun"), ("walk", "verb"), ("talk", "verb"), ("brush", "verb"), ("beautiful", "adjective")];
    let length_of_list = list_of_words.len();
    let rand_num = rand::thread_rng().gen_range(0..length_of_list);
    let (rand_word, word_type) = list_of_words[rand_num];

    // Create a string of underscores with the same length as the chosen word
    let mut hidden_word: Vec<char> = vec!['_'; rand_word.len()];

    let mut attempts = 6; // Number of attempts allowed
    let mut guessed_letters: Vec<char> = Vec::new(); // Store guessed letters

    println!("Welcome to Hangman!");
    println!("Guess the word:");

    // Display a hint
    println!("Hint: It's a {}", word_type);

    loop {
        // Display hidden word with guessed letters
        println!("{}", hidden_word.iter().collect::<String>());

        // Prompt the player to guess a letter
        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim().chars().next().expect("Please enter a letter");

        // Check if the guessed letter has already been guessed
        if guessed_letters.contains(&guess) {
            println!("You've already guessed that letter. Try again.");
            continue;
        }

        // Add the guessed letter to the list of guessed letters
        guessed_letters.push(guess);

        // Check if the guessed letter is in the word
        if !rand_word.chars().any(|c| c == guess) {
            attempts -= 1;
            println!("Incorrect guess! Attempts remaining: {}", attempts);
        } else {
            // Update the hidden word with the guessed letter
            for (i, c) in rand_word.chars().enumerate() {
                if c == guess {
                    hidden_word[i] = c;
                }
            }
        }

        // Check if the player has won or lost
        if hidden_word.iter().all(|&c| c != '_') {
            println!("Congratulations! You guessed the word: {}", rand_word);
            break;
        } else if attempts == 0 {
            println!("You ran out of attempts! The word was: {}", rand_word);
            break;
        }
    }
}