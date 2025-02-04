// Generate the passphrase
use std::collections::HashMap;
use std::string::String;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

static RANDOMIZED_VEC_SIZE: usize = 20_000_000;

pub fn get_passphrase(num_words: usize, delimiter: String) -> String {
    
    // get words_to_hashmap from <filepath>, see if I can denote filepath in a config file.
    // get a randomized vector
    // generate the passphrase

    let placeholder = String::from("Placeholder string to stop the compiler from throwing an err right now.");
}

fn words_to_hashmap(file_path: &str) -> io::Result<HashMap<usize, String>> {
    // Open the file in read-only mode
    let file = File::open(Path::new(file_path))?;
    let reader = io::BufReader::new(file);

    // Create an empty HashMap to store the words
    let mut word_map: HashMap<usize, String> = HashMap::new();

    // Iterate over each line in the file
    for (index, line) in reader.lines().enumerate() {
        // Unwrap the line or return an error if reading fails
        let word = line?;
        // Insert the word into the HashMap with its corresponding index
        word_map.insert(index, word);
    }

    Ok(word_map)
}

// return a vector of random integers. Each int represents a word from the words.txt file (the hashmap we create when we extract the words)
fn randomized_vector(range: usize) -> Vec<usize> {

    let mut random_ints: Vec<usize> = Vec::<usize>::new();

    for _i in 0..RANDOMIZED_VEC_SIZE {
        let rand_num = rand::thread_rng().gen_range(0..range);
        random_ints.push(rand_num);
    }

    let mut random_ints2: Vec<usize> = Vec::<usize>::new();

    for _i in 0..RANDOMIZED_VEC_SIZE {
        let rand_num = rand::thread_rng().gen_range(0..range);
        random_ints2.push(rand_num);
    }

    for _i in 0..RANDOMIZED_VEC_SIZE {

        let index = rand::thread_rng().gen_range(0..RANDOMIZED_VEC_SIZE);
        let which_rand = rand::thread_rng().gen_range(0..3);
        
        if which_rand == 0 {
            continue;
        } else if which_rand == 1 {
            random_ints[index] = random_ints[index];
        } else if which_rand == 2 {
            random_ints[index] = rand::thread_rng().gen_range(0..range);
        }
    }

    random_ints
}

// TODO -> Generate a random passphrase given the inputs
fn generate_passphrase(words_dict: HashMap<usize, String>, rand_vec: Vec<usize>, passphrase_length: usize) -> String {

}