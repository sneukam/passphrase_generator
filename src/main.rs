// A simple program
mod passphrase;
use clap::Parser;

static MIN_WORDS: usize = 6;
static MAX_WORDS: usize = 64;

// A simple program
#[derive(Parser)]
#[command(about="\n\nGenerate a passphrase. Dictionary is 9,200 words. Not compliant with any crypto standard. Default delimiter between words is a space to make it easier to type on a phone.")]
struct Cli {
    #[arg(short, long, default_value = "10")]
    num_words: String,

    #[arg(short, long, default_value = " ")]
    delimiter: String
}

fn main() {    
    let args: Cli = Cli::parse();
    let num_words: usize = get_num_words(args.num_words);
    let delimiter: String = args.delimiter;

    passphrase::print_passphrases(num_words, delimiter);
}

// Return the number of words to use in the passphrase as an integer. Throw an error if the number is outside the range of MIN_WORDS to MAX_WORDS (inclusive)
fn get_num_words(num_words: String) -> usize {

    let length: usize = num_words.parse::<usize>().unwrap_or_else(|_| panic!("Error: Please enter a positive, reasonably-sized number"));

    if length < MIN_WORDS || length > MAX_WORDS {
        panic!("Error: Please enter a number between {} and {}", MIN_WORDS, MAX_WORDS);
    };

    length
}
