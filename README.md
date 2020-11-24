# Phonetic Normalizer

This tool normalizes the spelling of words to make computer matching easier.

There are a number of other algorithms intended for this purpose. Most of these algorithms, like **Soundex** and **Caverphone**, focus on proper names. **Metaphone**, **Double Metaphone** and **Eudex** are all algorithms that are meant to work on both names and dictionary words. These are the most promising existing algorithms for our purpose and you can experiment with them using the [Talisman javascript library](https://yomguithereal.github.io/talisman/phonetics/).

## Why make a new algorithm?

In my testing of the above mentioned algorithms and a few others, I was disappointed by the results. For example, metaphone oversimplifies words, which creates matches where there shouldn't be any. For example, the words `color`, `colour`, and `cooler` all reduce to `KLR`.  I want color and colour to be considered the same, but I don't believe `cooler` should match either of them.

Eudex had the most promising approach as it creates an intelligent score for a word (they call it a hash). That number should differ only in the lower bits if it is similar to another word.  I attempted to discard the lower bits of the hash for matching purposes in testing.  Unfortunately, Eudex doesn't deal well with alternate spellings of names and considers many combinations of characters to be quite different -- especially if they come early in the word. Consequently I was unable to tune the number of bits dropped to get the results I wanted and in the end there was no way to fix for spellings that start a word differently like `catherine` and `kathryn`, which are considered extremely different words (high bits are different) by Eudex even though they sound the same.

## Goal

Our goal is to generally match homonyms and to correct for common misspellings and british/american spelling differences.  For our purposes, the normalized result could be a number instead of a word since we don't intend to show the normalized form to users and we don't care if it is reversible.

Our intent is to allow for fuzzy matches against a search index even if the word is encrypted and the server has no knowledge of what is being searched.  We want to produce the best search results possible by normalizing both the query and the index and then doing exact matches only.

Ideally, in testing across an entire dictionary of valid spellings, the only collisions will be between different forms of the same word or british/american spelling differences. Also, we want to properly identify most if not all of a [common misspellings](https://www.lexico.com/grammar/common-misspellings) list and most or all of the common [british/english spelling disagreements](http://www.tysto.com/uk-us-spelling-list.html). 

And the algorithm should be reasonably efficient with minimal string allocations and minimal passes over a word.

## Approach

In the end I was inspired by the Caverphone algorithm and its approach, but wanted to make it suitable for more general purposes.  The approach is to take common phonemes that can be spelled differently and to give them one representation. We remove all double letters except `ee` and `oo` and normalize most other double vowels (like `ie` vs. `ei` and `ea` vs. `ee`). We also normalize similar sounding letters like `b` and `p` and things that make a `ck` sound reduce everywhere to just a `k`.

Ultimately this is just a series of ordered substitutions.

## Measure of success

The algorithm seems to perform well against our test cases. It isn't perfect, but has a low false positive rate and is still highly likely to equate homonyms and misspellings.

Here are some of the words that we designed for and that will trigger a match. The third column shows the currently generated normalized word.

| Word  | Alternate | Phonetically Normalized Version |
| ----- | --------- | --------------------------------|
| catherine | kathryn | kathrine |
| john | jon | jon |
| cough | coff | kof |
| piece | peace | peese |
| mist | missed | mist |
| phone | fone | fone |
| phony | fony | fony |
| accomodate | accommodate | acomotite |
| achieve | acheive | asheeve |
| apparent | apparant | aprent |
| basically | basicly | basicly |
| argument | arguement | argument |
| definitely | definately | definatly |
| fourty | forty | forty |
| further | farther | frthr |
| gist | jist | gist |
| byte | bite | bite |
| gray | grey | gray |
| siege | seige | seege |
| sense | sence | sense |
| consonant | consonent | konsonent |
| shaq | shack | shak |
| gnat | nat | nat |
| knight | night | nite |
| night | nite | nite |
| knit | nit | nit |
| gnaw | naw | naw |
| natural | nateral | natrl |
| wherever | whereever | wrevr |
| color | colour | kolor |
| accessorise | accessorize | asesorise |
| abhominable | abominable | apominaple |
| curiousity | curiosity | kriosity |
| aerogramme | aerogram | arogram |
| almanack | almanac | almenak |
| anaemia | anemia | anemia |
| archaeology | archeology | asheology |
| behavioural | behavioral | beaviorl |
| cancellation | cancelation | kenselation |

And the words in the following table produce mismatches, which is what we want:

| Word1 | Word2 |
| ----- | ----- |
| at | ate |
| color | cooler |
| phony | phone |
| John | gone |
| precede | preset |
| rupert | robert |

## Using the library

This library is not (yet?) in crates.io. You'll need to clone it and build it yourself.

It is up to the caller to split words into their parts and to stem words (remove plurals, past tense, etc.) if desired before passing into the normalizer.  The only exported function is `normalize_word` and it will take a word and either return a reference back to it unchanged, or return a new string with the normalized word.  We also provide a simple command line utility for convenience of testing so you can feed dictionary files through it and see the incoming word next to the normalized word.  Here's a basic way to use the tool on a string containing multiple words:

```rust
use phonetic_normalizer::normalize_word;

fn your_func(lots_of_words: &str) {
  for word in lots_of_words.split_whitespace() {
    let normalized = normalize_word(&word);
    println!("{}\t{}", word, normalized);
  }
}
```

To use the command line tool, first build with `cargo b --release` and then do something like this:

```bash
> echo color colour cooler | ./target/release/phonetic-normalizer
color   kolor
colour  kolor
cooler  kolr
```

And that's it. File issues or PRs if you spot problems or want to add test cases.