//! This crate justifies plaintext for display in a terminal emulator in a  (mostly)
//! Unicode friendly way.
//!
//! **Examples of use can be found in the file `tests/tests.rs`.**
//!
//! If the crate is compiled with the `unicode-width` feature (e.g. via `cargo build
//! --features  unicode-width`), Unicode is handled gracefully. With this feature, a
//! CJK  character  such as 한 takes two spaces, while combining characters take  0.
//! Without this feature, every Unicode character takes one space, which can lead to
//! poor  output  in  some  cases.  If you will only ever  justify  ASCII  text,  or
//! NFC-normalized Unicode text of Latin languages, you don't need the feature.
//!
//! The width information is provided by the `wcwidth` crate.
//!
//! Without `unicode-width` (example text from
//! [here](https://en.wikipedia.org/wiki/Korea#Etymology)):
//!
//! ```text
//! "Korea"  is the modern spelling of "Corea", a name attested in English as  early
//! as  1614.[citation  needed] Korea was transliterated as Cauli in The Travels  of
//! Marco  Polo,[10] based on the kingdom of Goryeo (Hangul: 고려; Hanja:  高麗;
//! MR:  Koryŏ), which ruled most of the Korean peninsula during Marco Polo's time.
//! Korea's  introduction to the West resulted from trade and contact with merchants
//! from  Arabic  lands,[11]  with  some  records dating back  as  far  as  the  9th
//! century.[12]  Goryeo's  name  was  a continuation  of  Goguryeo  (Koguryŏ)  the
//! northernmost  of  the  Three Kingdoms of Korea, which was  officially  known  as
//! Goryeo  beginning in the 5th century.[13] The original name was a combination of
//! the  adjective  go ("high, lofty") with the name of a local Yemaek tribe,  whose
//! original  name  is  thought to have been either *Guru  (溝樓,  "walled  city,"
//! inferred   from  some  toponyms  in  Chinese  historical  documents)  or  *Gauri
//! (가우리, "center").
//! ```
//!
//! With `unicode-width` and `wcwidth: true` in `Settings` struct:
//!
//! ```text
//! "Korea"  is the modern spelling of "Corea", a name attested in English as  early
//! as  1614.[citation  needed] Korea was transliterated as Cauli in The Travels  of
//! Marco  Polo,[10] based on the kingdom of Goryeo (Hangul: 고려; Hanja: 高麗;  MR:
//! Koryŏ),  which  ruled  most of the Korean peninsula during  Marco  Polo's  time.
//! Korea's  introduction to the West resulted from trade and contact with merchants
//! from  Arabic  lands,[11]  with  some  records dating back  as  far  as  the  9th
//! century.[12]  Goryeo's  name  was  a  continuation  of  Goguryeo  (Koguryŏ)  the
//! northernmost  of  the  Three Kingdoms of Korea, which was  officially  known  as
//! Goryeo  beginning in the 5th century.[13] The original name was a combination of
//! the  adjective  go ("high, lofty") with the name of a local Yemaek tribe,  whose
//! original  name  is  thought  to have been either  *Guru  (溝樓,  "walled  city,"
//! inferred  from some toponyms in Chinese historical documents) or *Gauri (가우리,
//! "center").
//! ```
//!
//! Notice  that  the  justification is better with `unicode-width`, but  there  are
//! still  lines where the justification is one off. That's because it's not  always
//! possible  to  justify perfectly: as Korean characters take two terminal  spaces,
//! and  Latin  letters  take one, it's possible for there to be an  odd  number  of
//! characters  on  a line to be justified. Also, depending on your browser, it  may
//! not look right, try pasting it into a terminal emulator.

#[cfg(feature="unicode-width")] extern crate unicode_width;
#[cfg(feature="unicode-width")] use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Where to insert spaces (use with `Settings`)
pub enum InsertAt<'a> {
    /// Spaces are added starting at the left.
    Left,
    /// Spaces are added starting at the right.
    Right,
    /// Default;  e.g.  if there are 5 places spaced could be added,  the  first
    /// space  goes in place 1, the second space in place 5, the third space  in
    /// place 2, fourth space in place 4, etc.
    Balanced,
    /// The function receives the current 0-indexed iteration in position 1, the
    /// total number of spaces to be added in position 2, the number of possible
    /// entry  points in position 3, and the line being justified in position 4.
    /// This  could  be used, for example, to implement insertion of  spaces  at
    /// random  points. If using this, you may not need every argument, but they
    /// are provided anyway for maximum extensibility.
    Custom(&'a Fn(usize, usize, usize, &Vec<&str>)->usize)
}

/// Settings used by `justify` and `justify_paragraph`
pub struct Settings<'a> {
    /// Whether the last line should also be justified. Can result in weird output if the last line
    /// contains very few words.
    pub justify_last_line: bool,
    /// Hyphenate if a word is longer than `self.width`
    pub hyphenate_overflow: bool,
    /// Width (in codepoints)
    pub width: usize,
    /// In a given line, the pattern spaces should be inserted at.
    pub insert_at: InsertAt<'a>,
    #[cfg(feature="unicode-width")]
    /// On unicode text, attempt to use wcwidth
    pub wcwidth: bool,
    /// This feature is sometimes useful with CJK text in conjunction with hyphenate_overflow. When
    /// on, spaces are not considered when justifying text.
    pub ignore_spaces: bool,
    /// The string that should be used to separate lines. Perhaps useful on Windows where you might
    /// want "\r\n" instead.
    pub newline: &'a str,
    /// The hyphen that should be used if `hyphenate_overflow` is true
    pub hyphen: &'a str,
    /// The separator between paragraphs when `justify` is called
    pub separator: &'a str
}

impl<'a> Default for Settings<'a> {
    fn default() -> Self {
        Settings {
            justify_last_line: false,
            width: 80,
            hyphenate_overflow: false,
            insert_at: InsertAt::Balanced,
            #[cfg(feature="unicode-width")]
            wcwidth: false,
            ignore_spaces: false,
            newline: "\n",
            hyphen: "-",
            separator: "\n\n"
        }
    }
}

/// Generate where we should break and put it into v, like
/// vec![0, 12, 26, 40, 52, 65]
fn get_break_indexes(words: &Vec<&str>, settings: &Settings) -> Vec<usize> {
    let mut n = 0;
    let mut v = Vec::with_capacity(words.len()/4);
    v.push(0);

    for (i, word) in words.iter().enumerate() {
        let mut c;
        #[cfg(feature="unicode-width")] {
        if settings.wcwidth {
            c = n + word.width();
        } else {
            c = n + word.len();
        }
        }
        #[cfg(not(feature="unicode-width"))] {
            c = n + word.len();
        }
        if word.len() == 0 { continue }
        // If the last character in the word is whitespace, we have to ignore it in the
        // comparison, otherwise lines which are exactly the right width will be broken
        // as if they were one character too long.
        let cc = word.chars().nth(word.len()-1);
        if c - if cc.map_or(false, char::is_whitespace) { 1 } else { 0 } > settings.width {
            v.push(i);
            n = word.len();
        } else {
            n = c;
        }
    }

    v
}

fn lines_from_indexes<'a>(words: &Vec<&'a str>, breaks: &Vec<usize>) -> Vec<Vec<&'a str>> {
    let mut lines: Vec<Vec<&str>> = Vec::with_capacity(breaks.len());

    for i in 0..breaks.len()-1 {
        let mut t_v = Vec::from(&words[breaks[i]..breaks[i+1]]);
        let t_l = t_v.len();
        // Chop the final " " off of the last string in a line
        // last element of t_v = last element of t_v[0..length of last element of t_v-1]
        if t_v.len() == 0 { continue }
        t_v[t_l-1] = &t_v[t_l-1][0..&t_v[t_l-1].len()-1];
        lines.push(t_v);
    }

    // Handle last line
    lines.push(Vec::from(&words[breaks[breaks.len()-1]..]));

    lines
}

/// Determines how many spaces need to be added to the line to get it to width.
fn spaces_to_add(lines: &Vec<Vec<&str>>, settings: &Settings) -> Vec<usize> {
    let mut spaces: Vec<usize> = Vec::with_capacity(lines.len());

    for line in lines.iter() {
        let mut size = line.iter().fold(0, |acc, &x| acc + x.len());
        #[cfg(feature="unicode-width")]
        match settings.wcwidth {
            true => {size = line.iter().fold(0, |acc, &x| acc + x.width())},
            false => {}
        }

        if settings.width < size {
            spaces.push(0);
        } else {
            spaces.push(settings.width - size);
        }
    }

    spaces
}

/// Adds the spaces. Should be used with `spaces_to_add`
fn add_spaces(add: usize, line: &Vec<&str>, insert_at: &InsertAt) -> String {
    if line.len() == 0 { return String::new() }
    let v_i = line.len()-1;
    let mut add_v = vec![0; v_i];

    if v_i == 0 {
        return line[0].to_owned()
    }

    match *insert_at {
        InsertAt::Left => {
            for j in (1..v_i+1).into_iter().cycle().take(add) {
                add_v[j-1] += 1;
            }
        },
        InsertAt::Right => {
            for j in (1..v_i+1).rev().into_iter().cycle().take(add) {
                add_v[j-1] += 1;
            }
        },
        InsertAt::Balanced => {
            for j in (1..v_i+1).into_iter().cycle().take(add) {
                if j % 2 == 0 { //EVEN
                    add_v[v_i - (j/2)] += 1;
                } else { //ODD
                    add_v[(j/2)] += 1;
                }
            }
        },
        InsertAt::Custom(f) => {
            for j in 0..add {
                add_v[f(j, add, v_i, line)] += 1;
            }
        }
    }

    let space_s: Vec<String> = add_v.iter()
        .map(|i|" ".repeat(*i))
        .collect();

    // Length of spaces
    let space_l: usize = add_v.iter().sum();
    // Length of text in line
    let line_l: usize = line.iter().map(|e|e.len()).sum();

    line.iter()
        .enumerate()
        .fold(
            String::with_capacity(space_l + line_l),
            |acc, (i, x)| {
                if i < line.len()-1 {
                    acc + x + &space_s[i]
                } else {
                    acc + x
                }
            }
        )
}

/// This function is needed because there is no better way(?) to split a string such that the sum
/// of the lengths of the output equals the length of the input.  That is to say: "e
/// e".split(char::is_whitespace) returns vec!["e", "e"] while we want vec!["e ", "e"]
fn split_into_words(text: &str) -> Vec<&str> {
    let zero = vec![0];

    let indices: Vec<_> = zero.into_iter()
        .chain(
            text.match_indices(char::is_whitespace)
            .map(|(i, _)|i+1)
            )
        .collect();

    let mut wwords = Vec::with_capacity(indices.len());

    for i in 0..indices.len()-1 {
        let t = &text[indices[i]..indices[i+1]];
        if !t.chars().all(char::is_whitespace) {
            wwords.push(t);
        }
    }

    wwords.push(&text[indices[indices.len()-1]..]);

    wwords
}

#[cfg(feature="unicode-width")]
fn hyphenate_overflow(text: &str, settings: &Settings) -> String {
    let mut ret = String::with_capacity(text.len());
    let sws: Vec<_>;
    let joiner: &str;
    if settings.ignore_spaces {
        sws = text.split(settings.newline).collect();
        joiner = settings.newline;
    } else {
        sws = text.split_whitespace().collect();
        joiner = " ";
    }
    let tl = sws.len();

    for (i, s) in sws.iter().enumerate() {
        if s.len() > settings.width {
            let h = s.chars()
                .collect::<Vec<_>>();

            let widths: Vec<usize> = h.iter()
                .map(|e| e.width().unwrap_or(0))
                .collect();

            let mut q = 0;
            let mut hq = vec![0];
            for (i, w) in widths.into_iter().enumerate() {
                q += w;
                if q > settings.width-(settings.hyphen.len()) {
                    hq.push(i);
                    q=w;
                }
            }

            let mut hhq = Vec::new();
            for e in hq.windows(2) {
                if e.len() == 2 {
                    hhq.push(&h[e[0]..e[1]]);
                } else {
                    continue
                }
            }
            hhq.push(&h[*hq.last().unwrap()..]);

            let mut hh = hhq.iter().peekable();

            let mut f: Vec<String> = Vec::new();
            loop {
                let s: String = hh.next().unwrap().iter().collect();
                if hh.peek().is_some() {
                    f.push(s + settings.hyphen);
                } else {
                    f.push(s);
                    break
                }
            }

            ret += &f.join(joiner);
        } else {
            ret += s;
        }
        if i != tl-1 {
            ret += joiner;
        }
    }

    ret
}

#[cfg(not(feature="unicode-width"))]
fn hyphenate_overflow(text: &str, settings: &Settings) -> String {
    let mut ret = String::with_capacity(text.len());
    let sws: Vec<_>;
    let joiner: &str;
    if settings.ignore_spaces {
        sws = text.split(settings.newline).collect();
        joiner = settings.newline;
    } else {
        sws = text.split_whitespace().collect();
        joiner = " ";
    }
    let tl = sws.len();

    for (i, s) in sws.iter().enumerate() {
        if s.len() > settings.width {
            let h = s.chars().collect::<Vec<_>>();

            let mut f: Vec<String> = Vec::new();
            let mut p = h.chunks(settings.width-(settings.hyphen.len())).peekable();

            loop {
                let s: String = p.next().unwrap().iter().collect();
                if p.peek().is_some() {
                    f.push(s + settings.hyphen);
                } else {
                    f.push(s);
                    break
                }
            }

            ret += &f.join(joiner);
        } else {
            ret += s;
        }
        if i != tl-1 {
            ret += joiner;
        }
    }

    ret
}

/// Justify a single paragraph. Panics if "paragraph" contains newlines.
pub fn justify_paragraph(text: &str, settings: &Settings) -> String {
    if text.contains("\n") {
        panic!("Expected `text` to contain no newlines but it did")
    }

    let mut ret = String::with_capacity(text.len() + (text.len() / 3));

    let words = split_into_words(text);
    //eprintln!("W:{:?}",words);
    let breaks = get_break_indexes(&words, &settings);
    //eprintln!("B:{:?}",breaks);
    let lines = lines_from_indexes(&words, &breaks);
    //eprintln!("L:{:?}",lines);
    let spaces = spaces_to_add(&lines, &settings);
    //eprintln!("S:{:?}",spaces);

    for (i, space) in spaces.iter().enumerate() {
        if !settings.justify_last_line && i == spaces.len() - 1 {
            ret += &lines[spaces.len()-1].join("");
            break
        }
        if !settings.ignore_spaces {
            let add = &add_spaces(*space, &lines[i], &settings.insert_at);
            ret += add;
        } else {
            ret += &lines[i].join(" ");
        }
        ret += settings.newline;
    }

    ret
}

/// Justify `text` according to the parameters in `settings`.
pub fn justify(text: &str, settings: &Settings) -> String {
    let mut h = String::new();
    if settings.hyphenate_overflow {
        h = hyphenate_overflow(text, &settings);
    }

    if settings.ignore_spaces {
        return h;
    }

    if settings.hyphenate_overflow { h.as_str() } else { text }
        .split(settings.newline)
        .filter(
            |e|e.len()!=0
            )
        .map(
            |p| justify_paragraph(p, settings)
            )
        .collect::<Vec<_>>()
        .join(settings.separator)
}
