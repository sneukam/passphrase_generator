// Generate the passphrase
use std::collections::HashMap;
use std::string::String;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

static NUM_PASSPHRASES_TO_GENERATE: usize = 7;
static RANDOMIZED_VEC_SIZE: usize = 20_000_000;
static WORDS_TXT_FILEPATH: &str = "./words/words.txt";

pub fn print_passphrases(passphrase_length: usize, delimiter: String) {

    let words = words_to_hashmap(WORDS_TXT_FILEPATH).expect("Failed to read file.");
    let word_dictionary_size = words.iter().count();
    let randomized_vec = randomized_vector(RANDOMIZED_VEC_SIZE, word_dictionary_size);
    
    println!("\nDictionary: {} words", word_dictionary_size);
    println!("Passphrase length: {}", passphrase_length);
    println!("Bits of entropy: {}\n", passphrase_entropy(passphrase_length, word_dictionary_size));

    for _i in 0..NUM_PASSPHRASES_TO_GENERATE {
        let passphrase = generate_passphrase(words.clone(), randomized_vec.clone(), passphrase_length, delimiter.clone());
        println!("{}", passphrase);
    }
}

// Return a file result that contains a HashMap<u16, String> of all the words from the words.txt with their index line location.
fn words_to_hashmap(file_path: &str) -> io::Result<HashMap<u16, String>> {
    // Open the file in read-only mode
    let file = File::open(Path::new(file_path))?;
    let reader = io::BufReader::new(file);

    // Create an empty HashMap to store the words
    let mut word_map: HashMap<u16, String> = HashMap::new();

    // Iterate over each line in the file
    for (index, line) in reader.lines().enumerate() {
        // Unwrap the line or return an error if reading fails
        let word = line?;
        // Insert the word into the HashMap with its corresponding index
        word_map.insert(index as u16, word);
    }

    Ok(word_map)
}

// return a vector of random integers. Each int represents a word from the words.txt file, where the int maps to a word via the hashmap built from words.txt
fn randomized_vector(randomized_vec_size: usize, word_range: usize) -> Vec<u16> {

    let mut random_ints: Vec<u16> = Vec::<u16>::new();

    for _i in 0..randomized_vec_size {
        let rand_num = rand::thread_rng().gen_range(0..word_range) as u16;
        random_ints.push(rand_num);
    }

    let mut random_ints2: Vec<u16> = Vec::<u16>::new();

    for _i in 0..randomized_vec_size {
        let rand_num = rand::thread_rng().gen_range(0..word_range) as u16;
        random_ints2.push(rand_num);
    }

    for _i in 0..randomized_vec_size {

        let index = rand::thread_rng().gen_range(0..randomized_vec_size);
        let which_rand = rand::thread_rng().gen_range(0..3) as u8;
        
        if which_rand == 0 {
            continue;
        } else if which_rand == 1 {
            random_ints[index] = random_ints2[index];
        } else if which_rand == 2 {
            random_ints[index] = rand::thread_rng().gen_range(0..word_range) as u16;
        }
    }

    random_ints
}

fn passphrase_entropy(passphrase_length: usize, word_dictionary_size: usize) -> usize {
    if word_dictionary_size == 0 {
        panic!("Word dictionary size must be greater than 0.");
    }

    // Calculate the entropy using the formula: log2(word_dictionary_size) * passphrase_length
    let entropy_per_word = (word_dictionary_size as f64).log2();
    let total_entropy = (entropy_per_word * (passphrase_length as f64)).floor() as usize;

    total_entropy
}

// Return a random passphrase
fn generate_passphrase(words: HashMap<u16, String>, randomized_vec: Vec<u16>, passphrase_length: usize, delimiter: String) -> String {
    
    let vec_size = randomized_vec.iter().len();
    let mut words_generated: usize = 0;
    let mut passphrase: String = String::new();

    while words_generated < passphrase_length {

        let vec_index= rand::thread_rng().gen_range(0..vec_size);
        let chance = rand::thread_rng().gen_range(0..10) as usize;
        let y_n = rand::thread_rng().gen_range(0..10) as usize;

        if chance == y_n {
            let word_index = randomized_vec[vec_index];
            passphrase += &words[&word_index];
            words_generated += 1;
            if words_generated < passphrase_length {
                passphrase += &delimiter;
            }
        }
    }

    passphrase
}
