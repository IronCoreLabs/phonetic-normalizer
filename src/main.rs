use phonetic_normalizer::normalize_word;
use std::io::{self, Read};

fn main() -> io::Result<()> {
  let mut buffer = String::new();
  let mut stdin = io::stdin(); // We get `Stdin` here.
  stdin.read_to_string(&mut buffer)?;
  for word in buffer.split_whitespace() {
    let normalized = normalize_word(&word);
    println!("{}\t{}", word, normalized);
  }
  Ok(())
}
