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
    let mut random_ints2: Vec<u16> = Vec::<u16>::new();

    // linearly fill with random numbers
    for _i in 0..randomized_vec_size {
        let rand_num = rand::thread_rng().gen_range(0..word_range) as u16;
        let rand_num2 = rand::thread_rng().gen_range(0..word_range) as u16;

        random_ints.push(rand_num);
        random_ints2.push(rand_num2);
    }

    // place random numbers at random indices
    for _i in 0..randomized_vec_size {
        let rand_index = rand::thread_rng().gen_range(0..randomized_vec_size);
        let rand_index2 = rand::thread_rng().gen_range(0..randomized_vec_size);

        random_ints[rand_index] = rand::thread_rng().gen_range(0..word_range) as u16;
        random_ints2[rand_index2] = rand::thread_rng().gen_range(0..word_range) as u16;
    }

    // randomly select a vector to return
    if rand::thread_rng().gen::<bool>() as usize == 1 {
        return random_ints;
    }
    random_ints2
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

    // build the passphrase
    while words_generated < passphrase_length {

        // either of vec_index or vec_index 2 will be used to select the position in randomized_vec where the next word is pulled from
        let vec_index= rand::thread_rng().gen_range(0..vec_size);
        let vec_index2 = rand::thread_rng().gen_range(0..vec_size);
        let chance1 = rand::thread_rng().gen_range(0..10) as usize;
        let chance2 = rand::thread_rng().gen_range(0..10) as usize;

        // Not ever loop iteration will add a word to the passphprase. 
        // vec_index and vec_index2 are intentionally created each loop before a word is potentially added.
        // This buffers against linear random generation, increasing complexity.
        if chance1 == chance2 {

            let rand_word_index = randomized_vec[vec_index];
            let rand_word_index2 = randomized_vec[vec_index2];
            let choice: u8 = rand::thread_rng().gen::<bool>() as u8;

            // Assign the value based on the random choice
            let word_index = match choice {
                0 => rand_word_index,
                1 => rand_word_index2,
                _ => unreachable!(), // This branch is unreachable since we know choice is either 0 or 1
            };
            
            passphrase += &words[&word_index];  // add the word to the passphrase
            words_generated += 1;

            // add a delimiter to the passphrase if there are still more words to add.
            if words_generated < passphrase_length {
                passphrase += &delimiter;
            }
        }
    }

    passphrase
}
