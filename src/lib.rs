use std::borrow::Cow;

pub fn normalize_word(source: &str) -> Cow<str> {
    let mut dest = source.to_lowercase();

    // **Start of word substitutions**

    let mut char_iter = dest.chars();
    let first_char: Option<char> = char_iter.next();
    let second_char: Option<char> = char_iter.next();

    match (first_char, second_char) {
        // s/^c/k/;
        (Some('c'), _) => dest.replace_range(0..1, "k"),
        // s/^qu/k/;
        (Some('q'), Some('u')) => dest.replace_range(0..2, "k"),
        // s/^ph/f/;
        (Some('p'), Some('h')) => dest.replace_range(0..2, "f"),
        // s/^wh/w/;
        (Some('w'), Some('h')) => dest.replace_range(0..2, "w"),
        // s/^kn/n/;
        (Some('k'), Some('n')) => dest.replace_range(0..2, "n"),
        // s/^x/z/;
        (Some('x'), _) => dest.replace_range(0..2, "z"),
        // s/^gn/n/;
        (Some('g'), Some('n')) => dest.replace_range(0..2, "n"),
        // s/^j/g/;
        (Some('j'), _) => dest.replace_range(0..1, "g"),
        (_, _) => {}
    }

    // **End of word substitutions**
    let mut char_iter = dest.chars();
    let last_char: Option<char> = char_iter.next_back();
    let last_char2: Option<char> = char_iter.next_back();
    let last_char3: Option<char> = char_iter.next_back();
    let last_char4: Option<char> = char_iter.next_back();

    match (last_char4, last_char3, last_char2, last_char) {
        // s/ee$/y/;
        (_, _, Some('e'), Some('e')) => dest.replace_range((dest.len() - 2).., "y"),
        // vowel,c => vowel,k s/([aeiouy])c$/$1k/;
        (_, _, Some(v), Some('c')) => {
            if is_vowel(&v, true) {
                dest.replace_range((dest.len() - 1).., "k");
            }
        }
        // s/[ae]ly$/ly/;
        (_, Some('a'), Some('l'), Some('y')) | (_, Some('e'), Some('l'), Some('y')) => {
            dest.replace_range((dest.len() - 3).., "ly")
        }
        // s/mme$/m/;
        (_, Some('m'), Some('m'), Some('e')) => dest.replace_range((dest.len() - 2).., ""),
        // s/ey$/y/;
        (_, _, Some('e'), Some('y')) => dest.replace_range((dest.len() - 2).., "y"),
        // s/cy$/sy/;
        (_, _, Some('c'), Some('y')) => dest.replace_range((dest.len() - 2).., "sy"),
        // (consonent except y),d => consonent,ed
        (_, _, Some(v), Some('d')) => {
            // s/ed$/t/;
            if v == 'e' {
                dest.replace_range((dest.len() - 2).., "t");
            // s/([^aeiouy])d$/$1t/;
            } else if !is_vowel(&v, true) {
                dest.replace_range((dest.len() - 1).., "t");
            }
        }
        // s/ce$/se/;
        (_, _, Some('c'), Some('e')) => dest.replace_range((dest.len() - 2).., "se"),
        // s/yn$/ine/;
        (_, _, Some('y'), Some('n')) => dest.replace_range((dest.len() - 2).., "ine"),
        // s/ent$/ant/;
        (_, Some('e'), Some('n'), Some('t')) => dest.replace_range((dest.len() - 3).., "ant"),
        // s/ible$/able/;
        (Some('i'), Some('b'), Some('l'), Some('e')) => {
            dest.replace_range((dest.len() - 4).., "able")
        }
        // s/ious$/ous/;
        (Some('i'), Some('o'), Some('u'), Some('s')) => {
            dest.replace_range((dest.len() - 4).., "ous")
        }
        (_, _, _, _) => {}
    }
    // s/itly$/atly/;
    replace_end_if(&mut dest, "itly", "atly");

    // **Rest of word changes (everything but first char)**

    if dest.len() > 1 {
        let first_char: String = dest.chars().take(1).collect();

        // remove double letters
        dest = dest
            .chars()
            .fold(String::with_capacity(dest.len()), |mut acc, c| {
                if !acc.ends_with(c) {
                    acc.push(c);
                }
                // s/ight/ite/g;
                if acc.ends_with("ight") {
                    // This must be done in an early pass
                    replace_last(&mut acc, 4, "ite");
                }
                // s/our/or/g;
                if acc.ends_with("our") {
                    // This must be done in an early pass
                    replace_last(&mut acc, 3, "or");
                }
                // s/[uae]r/r/g;
                if acc.ends_with("ur") || acc.ends_with("ar") || acc.ends_with("er") {
                    // This must be done in an early pass
                    replace_last(&mut acc, 2, "r");
                }
                acc
            });

        let (new_dest, _, _) = dest.chars().skip(1).fold(
            (String::with_capacity(dest.len()), ' ', ' '),
            |(mut acc, c1, c2), c3| {
                match (c1, c2, c3) {
                    // s/([^aeiou])al/$1l/g;
                    (consonant, 'a', 'l') => {
                        if consonant == ' ' || is_vowel(&consonant, false) {
                            acc.push('l');
                        } else {
                            replace_last(&mut acc, 2, format!("{}l", consonant).as_str());
                        }
                    }
                    // s/igh/i/g;
                    ('i', 'g', 'h') => replace_last(&mut acc, 1, ""),
                    // s/gh/f/g;
                    (_, 'g', 'h') => replace_last(&mut acc, 1, "f"),
                    // s/eu/e/g;
                    (_, 'e', 'u') => {} // don't add the u
                    // s/ea/ee/g;
                    (_, 'e', 'a') => acc.push('e'),
                    // s/ei/ee/g;
                    (_, 'e', 'i') => acc.push('e'),
                    // s/ie/ee/g;
                    (_, 'i', 'e') => replace_last(&mut acc, 1, "ee"),
                    // s/gue/gu/g;
                    ('g', 'u', 'e') => {} // don't add the e
                    // s/ue/e/g;
                    (_, 'u', 'e') => replace_last(&mut acc, 1, "e"),
                    // s/au/ua/g;
                    (_, 'a', 'u') => replace_last(&mut acc, 1, "ua"),
                    // s/ai/ae/g;
                    (_, 'a', 'i') => acc.push('e'),
                    // s/ae/e/g;
                    (_, 'a', 'e') => replace_last(&mut acc, 1, "e"),
                    // s/gn/n/g;
                    (_, 'g', 'n') => replace_last(&mut acc, 1, "n"),
                    // s/(mn|nm)/m/g;
                    (_, 'm', 'n') => {} // don't add the n
                    (_, 'n', 'm') => replace_last(&mut acc, 1, "m"),
                    // s/sc/c/g;
                    (_, 's', 'c') => replace_last(&mut acc, 1, "c"),
                    // s/ou/o/g;
                    (_, 'o', 'u') => {} // don't add the u
                    // s/uo/o/g;
                    (_, 'u', 'o') => replace_last(&mut acc, 1, "o"),
                    // s/ate/ite/g;
                    ('a', 't', 'e') => replace_last(&mut acc, 2, "ite"),
                    // s/ph/f/g;
                    (_, 'p', 'h') => replace_last(&mut acc, 1, "f"),
                    // s/an/en/g;
                    (_, 'a', 'n') => replace_last(&mut acc, 1, "en"),
                    // s/ao/oa/g;
                    (_, 'a', 'o') => replace_last(&mut acc, 1, "oa"),
                    // s/y(.)/i$1/g; note: make sure this doesn't match at the end of the word
                    // only convert y to i in the middle
                    (_, 'y', v) => replace_last(&mut acc, 1, format!("i{}", v).as_str()),
                    // s/anc/enc/g;
                    ('a', 'n', 'c') => replace_last(&mut acc, 2, "enc"),
                    // s/gm/m/g;
                    (_, 'g', 'm') => replace_last(&mut acc, 1, "m"),
                    // s/cq/k/g;
                    (_, 'c', 'q') => replace_last(&mut acc, 1, "k"),
                    // s/ck/k/g;
                    (_, 'c', 'k') => replace_last(&mut acc, 1, "k"),
                    // s/qu/k/g;
                    (_, 'q', 'u') => replace_last(&mut acc, 1, "k"),
                    // s/ce/se/g;
                    (_, 'c', 'e') => replace_last(&mut acc, 1, "se"),
                    // s/t[sc]h/sh/g;
                    ('t', 's', 'h') | ('t', 'c', 'h') => replace_last(&mut acc, 2, "sh"),
                    // s/ch/sh/g;
                    (_, 'c', 'h') => replace_last(&mut acc, 1, "sh"),
                    // s/dg/g/g;
                    (_, 'd', 'g') => replace_last(&mut acc, 1, "g"),
                    // s/ore/or/g;
                    ('o', 'r', 'e') => {} // don't add the e
                    // s/([^sth]+)h/$1/g;
                    // get rid of all h's except for start and ch/sh/th
                    (_, p, 'h') => {
                        if p == 'c' || p == 's' || p == 't' {
                            acc.push('h');
                        }
                    }

                    _ => acc.push(c3),
                };

                (acc, c2, c3)
            },
        );
        dest = first_char.clone() + &new_dest;

        dest = first_char
            + &dest
                .chars()
                .skip(1)
                .fold(String::with_capacity(dest.len()), |mut acc, c| {
                    match c {
                        // s/q/k/g;
                        'q' => acc.push('k'),
                        // s/x/k/g;
                        'x' => acc.push('k'),
                        // s/b/p/g;
                        'b' => acc.push('p'),
                        // s/d/t/g;
                        'd' => acc.push('t'),
                        // s/z/s/g;
                        'z' => acc.push('s'),
                        _ => acc.push(c),
                    }
                    acc
                });
    }

    if source == dest {
        Cow::Borrowed(source)
    } else {
        Cow::Owned(dest)
    }
}

fn replace_start_if(haystack: &mut String, search_for: &str, replacement: &str) {
    replace_if(
        haystack,
        |h| h.starts_with(search_for),
        0,
        search_for.len(),
        replacement,
    );
}

fn replace_last(s: &mut String, n: usize, replacement: &str) {
    for _ in 0..n {
        s.pop();
    }
    s.push_str(replacement);
}

fn replace_end_if(haystack: &mut String, search_for: &str, replacement: &str) {
    if haystack.len() >= search_for.len() {
        let start = haystack.len() - search_for.len();
        replace_if(
            haystack,
            |h| h.ends_with(search_for),
            start,
            search_for.len(),
            replacement,
        );
    }
}

fn replace_if<F: Fn(&str) -> bool>(
    haystack: &mut String,
    search_fn: F,
    start_at: usize,
    len: usize,
    replacement: &str,
) {
    // if haystack[start_at..].starts_with(search_for) {
    if search_fn(&haystack[start_at..]) {
        haystack.replace_range(start_at..(start_at + len), replacement);
    }
}

fn is_vowel(c: &char, match_y: bool) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        'y' => match_y,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Places to find examples:
    // Common misspellings: https://www.lexico.com/grammar/common-misspellings
    // British spellings: http://www.tysto.com/uk-us-spelling-list.html

    #[test]
    fn start_and_end() {
        assert_eq!(normalize_word("phonee"), "fony");
        assert_eq!(normalize_word("caley"), "kaly");
    }

    #[test]
    fn name_matches() {
        assert_eq!(normalize_word("Catherine"), normalize_word("Kathryn"));
        assert_eq!(normalize_word("Philbert"), normalize_word("Filbert"));
        assert_eq!(normalize_word("Walsh"), normalize_word("Walch"));
        assert_eq!(normalize_word("John"), normalize_word("Jon"));
    }

    #[test]
    fn common_misspellings() {
        assert_eq!(normalize_word("cough"), normalize_word("coff"));
        assert_eq!(normalize_word("piece"), normalize_word("peace"));
        assert_eq!(normalize_word("mist"), normalize_word("missed"));
        assert_eq!(normalize_word("phone"), normalize_word("fone"));
        assert_eq!(normalize_word("phony"), normalize_word("fony"));
        assert_eq!(normalize_word("accomodate"), normalize_word("accommodate"));
        assert_eq!(normalize_word("achieve"), normalize_word("acheive"));
        assert_eq!(normalize_word("apparent"), normalize_word("apparant"));
        assert_eq!(normalize_word("basically"), normalize_word("basicly"));
        assert_eq!(normalize_word("argument"), normalize_word("arguement"));
        assert_eq!(normalize_word("definitely"), normalize_word("definately"));
        assert_eq!(normalize_word("fourty"), normalize_word("forty"));
        assert_eq!(normalize_word("further"), normalize_word("farther"));
        assert_eq!(normalize_word("gist"), normalize_word("jist"));
        assert_eq!(normalize_word("byte"), normalize_word("bite"));
        assert_eq!(normalize_word("siege"), normalize_word("seige"));
        assert_eq!(normalize_word("sense"), normalize_word("sence"));
        assert_eq!(normalize_word("consonant"), normalize_word("consonent"));
        assert_eq!(normalize_word("shaq"), normalize_word("shack"));
        assert_eq!(normalize_word("gnat"), normalize_word("nat"));
        assert_eq!(normalize_word("knight"), normalize_word("night"));
        assert_eq!(normalize_word("night"), normalize_word("nite"));
        assert_eq!(normalize_word("knit"), normalize_word("nit"));
        assert_eq!(normalize_word("gnaw"), normalize_word("naw"));
        assert_eq!(normalize_word("natural"), normalize_word("nateral"));
    }

    #[test]
    fn common_british_spellings() {
        assert_eq!(normalize_word("color"), normalize_word("colour"));
        assert_eq!(normalize_word("accessorise"), normalize_word("accessorize"));
        assert_eq!(normalize_word("abhominable"), normalize_word("abominable"));
        assert_eq!(normalize_word("curiousity"), normalize_word("curiosity"));
        assert_eq!(normalize_word("aerogramme"), normalize_word("aerogram"));
        assert_eq!(normalize_word("almanack"), normalize_word("almanac"));
        assert_eq!(normalize_word("anaemia"), normalize_word("anemia"));
        assert_eq!(normalize_word("archaeology"), normalize_word("archeology"));
        assert_eq!(normalize_word("behavioural"), normalize_word("behavioral"));
        assert_eq!(
            normalize_word("cancellation"),
            normalize_word("cancelation")
        );
        // assert_eq!(normalize_word("catalogue"), normalize_word("catalog"));
    }

    #[test]
    fn mismatches() {
        assert_ne!(normalize_word("at"), normalize_word("ate"));
        assert_ne!(normalize_word("color"), normalize_word("cooler"));
        assert_ne!(normalize_word("phony"), normalize_word("phone"));
        assert_ne!(normalize_word("John"), normalize_word("gone"));
        assert_ne!(normalize_word("precede"), normalize_word("preset"));
        assert_ne!(normalize_word("rupert"), normalize_word("robert"));
    }
}
