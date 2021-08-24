use std::borrow::Cow;

pub fn normalize_word(source: &str) -> Cow<str> {
    let mut dest = source.to_lowercase();

    // **Start of word substitutions**

    let mut char_iter = dest.chars();
    let first_char: Option<char> = char_iter.next();
    let second_char: Option<char> = char_iter.next();

    match (first_char, second_char) {
        // s/^c([^eh])/k$1/;
        (Some('c'), Some(v)) =>
        // turn a leading c into a k unless the second letter is e or h
        {
            if v == 'e' || v == 'i' {
                // ce at start of word is typically pronounced "se" like cent / sent
                // and ci at start of word pronounced "si" like cider, civil, citrus
                dest.replace_range(0..1, "s");
            } else if v == 'h' {
                // we have a leading ch -- leave it as-is
            } else {
                // otherwise assume a hard k sound
                dest.replace_range(0..1, "k");
            }
        }
        // s/^qu/k/;
        (Some('q'), Some('u')) => dest.replace_range(0..2, "k"),
        // s/^ph/f/;
        (Some('p'), Some('h')) => dest.replace_range(0..2, "f"),
        // s/^wh/w/;
        (Some('w'), Some('h')) => dest.replace_range(0..2, "w"),
        // s/^kn/n/;
        (Some('k'), Some('n')) => dest.replace_range(0..2, "n"),
        // s/^x/z/;
        (Some('x'), _) => dest.replace_range(0..1, "z"),
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
        (_, Some(v), Some('e'), Some('y')) => {
            if v == 'r' {
                // s/rey$/ray/;
                dest.replace_range((dest.len() - 2).., "ay");
            } else {
                // s/ey$/y/;
                dest.replace_range((dest.len() - 2).., "y");
            }
        }
        // s/cy$/sy/;
        (_, _, Some('c'), Some('y')) => dest.replace_range((dest.len() - 2).., "sy"),
        // (consonent except y),d => consonent,ed
        (_, _, Some(v), Some('d')) => {
            // s/ed$/d/;
            if v == 'e' {
                dest.replace_range((dest.len() - 2).., "d");
            // s/([^aeiouy])d$/$1t/;
            } else if !is_vowel(&v, true) {
                dest.replace_range((dest.len() - 1).., "t");
            }
        }
        // s/ce$/se/;
        (_, _, Some('c'), Some('e')) => dest.replace_range((dest.len() - 2).., "se"),
        // s/rine$/ine/;
        (Some('r'), Some('i'), Some('n'), Some('e')) => {
            dest.replace_range((dest.len() - 4).., "rin")
        }
        // s/yn$/ine/;
        (_, _, Some('y'), Some('n')) => dest.replace_range((dest.len() - 2).., "in"),
        // s/ent$/ant/;
        (_, Some('e'), Some('n'), Some('t')) => dest.replace_range((dest.len() - 3).., "ant"),
        // s/ien$/ian/;
        (_, Some('i'), Some('e'), Some('n')) => dest.replace_range((dest.len() - 2).., "an"),
        // s/ible$/able/;
        (Some('i'), Some('b'), Some('l'), Some('e')) => {
            dest.replace_range((dest.len() - 4).., "able")
        }
        // s/ious$/ous/;
        (Some('i'), Some('o'), Some('u'), Some('s')) => {
            dest.replace_range((dest.len() - 4).., "ous")
        }
        // s/itly$/atly/;
        (Some('i'), Some('t'), Some('l'), Some('y')) => {
            dest.replace_range((dest.len() - 4).., "atly")
        }
        // s/sean/shawn/g;
        (Some('s'), Some('e'), Some('a'), Some('n')) => {
            dest.replace_range((dest.len() - 4).., "shawn")
        }
        (_, _, _, _) => {}
    }
    // Must happen after other changes
    // s/itly$/atly/;
    replace_end_if(&mut dest, "itly", "atly");

    // **Rest of word changes (everything but first char)**

    if dest.len() > 1 {
        let first_char: String = dest.chars().take(1).collect();

        // Remove double letters. Don't skip first letter.
        dest = dest
            .chars()
            .fold(String::with_capacity(dest.len()), |mut acc, c| {
                // preserve ee and oo
                if !acc.ends_with(c) || c == 'e' || c == 'o' {
                    acc.push(c);
                }

                // These next few just need to happen before the full pass below.
                // We do a length check because we don't want to catch the first character
                // in these tests, which should only apply to middle and end of word matches.

                // s/ought/ot/g;
                if acc.len() > 5 && acc.ends_with("ought") {
                    replace_last(&mut acc, 5, "ot");
                }
                // s/plough/plow/g;
                if acc.len() > 5 && acc.ends_with("plough") {
                    replace_last(&mut acc, 6, "plow");
                }
                // s/dough/do/g;
                if acc.len() > 4 && acc.ends_with("dough") {
                    replace_last(&mut acc, 5, "do");
                }
                // s/ight/ite/g;
                if acc.len() > 4 && acc.ends_with("ight") {
                    // This must be done in an early pass
                    replace_last(&mut acc, 4, "ite");
                }
                // s/eagh/eg/g;
                if acc.len() > 4 && acc.ends_with("eagh") {
                    // This must be done in an early pass
                    replace_last(&mut acc, 4, "eg");
                }
                // s/eaga/ega/g;
                if acc.len() > 4 && acc.ends_with("eaga") {
                    // This must be done in an early pass
                    replace_last(&mut acc, 4, "ega");
                }
                // s/our/or/g;
                if acc.len() > 3 && acc.ends_with("our") {
                    // This must be done in an early pass
                    replace_last(&mut acc, 3, "or");
                }
                // We already did this for end of word, but need mid-word
                // and needs to be run early.
                // s/rey/ray/g;
                if acc.len() > 3 && acc.ends_with("rey") {
                    replace_last(&mut acc, 2, "ay");
                }
                // s/[uae]r/r/g;
                if acc.len() > 2
                    && (acc.ends_with("ur") || acc.ends_with("ar") || acc.ends_with("er"))
                {
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
                    // s/in/en/g;
                    (_, 'i', 'n') => replace_last(&mut acc, 1, "en"),
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
                        // keep the h if the preceding char is c or s or t
                        if p == 'c'
                            || p == 's'
                            || p == 't'
                            || (p == ' '
                                && (first_char == "c" || first_char == "t" || first_char == "s"))
                        {
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
            + &dest.char_indices().skip(1).fold(
                String::with_capacity(dest.len()),
                |mut acc, (byte_idx, c)| {
                    match c {
                        // s/q/k/g;
                        'q' => acc.push('k'),
                        // s/x/k/g;
                        'x' => acc.push('k'),
                        // s/z/s/g;
                        'z' => acc.push('s'),

                        // s/b/p/g;
                        'b' => {
                            // Only do this ones if we aren't on the last char
                            if byte_idx < dest.len() {
                                acc.push('p');
                            }
                        }
                        // s/d/t/g;
                        'd' => {
                            if byte_idx < dest.len() {
                                acc.push('t');
                            }
                        }

                        _ => acc.push(c),
                    }
                    acc
                },
            );
    }

    if source == dest {
        Cow::Borrowed(source)
    } else {
        Cow::Owned(dest)
    }
}

fn replace_last(s: &mut String, n: usize, replacement: &str) {
    for _ in 0..n {
        s.pop();
    }
    s.push_str(replacement);
}

fn replace_end_if(haystack: &mut String, search_for: &str, replacement: &str) {
    if haystack.ends_with(search_for) {
        let start_at = haystack.len() - search_for.len();
        haystack.replace_range(start_at..(start_at + search_for.len()), replacement);
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
    fn specific_matches() {
        assert_eq!(normalize_word("phonee"), "fony");
        assert_eq!(normalize_word("caley"), "kaly");
        assert_eq!(normalize_word("argument"), "argument");
        assert_eq!(normalize_word("shack"), "shak");
    }

    #[test]
    fn name_matches() {
        assert_eq!(normalize_word("Heriberto"), normalize_word("Hariberto"));
        assert_eq!(normalize_word("Catherine"), normalize_word("Kathryn"));
        assert_eq!(normalize_word("Philbert"), normalize_word("Filbert"));
        assert_eq!(normalize_word("Walsh"), normalize_word("Walch"));
        assert_eq!(normalize_word("John"), normalize_word("Jon"));
        assert_eq!(normalize_word("Gary"), normalize_word("Gery"));
        assert_eq!(normalize_word("Gary"), normalize_word("Jerry"));
        assert_eq!(normalize_word("Catie"), normalize_word("Katie"));
        assert_eq!(normalize_word("Megan"), normalize_word("Meaghan"));
        assert_eq!(normalize_word("Megan"), normalize_word("Meagan"));
        assert_eq!(normalize_word("Ashley"), normalize_word("Ashlee"));
        assert_eq!(normalize_word("Sara"), normalize_word("Sarah"));
        assert_eq!(normalize_word("Sienna"), normalize_word("Siena"));
        assert_eq!(normalize_word("Savanna"), normalize_word("Savannah"));
        assert_eq!(normalize_word("Alison"), normalize_word("Allison"));
        assert_eq!(normalize_word("Sofia"), normalize_word("Sophia"));
        assert_eq!(normalize_word("Grayson"), normalize_word("Greyson"));
        assert_eq!(normalize_word("Elliot"), normalize_word("Elliott"));
        assert_eq!(normalize_word("Collin"), normalize_word("Colin"));
        assert_eq!(normalize_word("Sebastian"), normalize_word("Sebastien"));
        assert_eq!(normalize_word("Sean"), normalize_word("Shawn"));
        assert_eq!(normalize_word("Julian"), normalize_word("Julien"));
        assert_eq!(normalize_word("Robyn"), normalize_word("Robin"));
        assert_eq!(normalize_word("Merlin"), normalize_word("Merlyn"));
        assert_eq!(normalize_word("Lauren"), normalize_word("Lauryn"));
    }

    #[test]
    fn common_misspellings() {
        assert_eq!(normalize_word("cough"), normalize_word("coff"));
        assert_eq!(normalize_word("bought"), normalize_word("bot"));
        assert_eq!(normalize_word("doughnut"), normalize_word("donut"));
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
        assert_eq!(normalize_word("aardvark"), normalize_word("ardvark"));
        assert_eq!(normalize_word("cent"), normalize_word("sent"));
        assert_eq!(normalize_word("cite"), normalize_word("site"));
        assert_eq!(normalize_word("gray"), normalize_word("grey"));
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
        assert_eq!(normalize_word("plough"), normalize_word("plow"));
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
        assert_ne!(normalize_word("shack"), normalize_word("sack"));
        assert_ne!(normalize_word("cent"), normalize_word("chant"));
        assert_ne!(normalize_word("cough"), normalize_word("cow"));
    }

    #[test]
    fn multibyte_chars() {
        assert_eq!(normalize_word("piétro"), "piétro");
        assert_eq!(normalize_word("piéitly"), "piéatly");
    }

    #[test]
    fn short_words() {
        assert_eq!(normalize_word("a"), "a");
        assert_eq!(normalize_word("be"), "be");
        assert_eq!(normalize_word("at"), "at");
        assert_eq!(normalize_word("do"), "do");
    }

    #[test]
    fn replace_end_if_tests() {
        let mut s = "word".to_string();
        replace_end_if(&mut s, "rd", "od");
        assert_eq!(s, "wood");

        let mut s = "word".to_string();
        replace_end_if(&mut s, "d", "ld");
        assert_eq!(s, "world");

        let mut s = "word".to_string();
        replace_end_if(&mut s, "rd", "e");
        assert_eq!(s, "woe");

        let mut s = "piétro".to_string();
        replace_end_if(&mut s, "iétro", "et");
        assert_eq!(s, "pet");

        let mut s = "piétro".to_string();
        replace_end_if(&mut s, "tro", "é");
        assert_eq!(s, "piéé");

        let mut s = "é".to_string();
        replace_end_if(&mut s, "é", "aaa");
        assert_eq!(s, "aaa");
    }
}
