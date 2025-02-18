use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use crate::DICTIONARY;
use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use regex::Regex;

#[allow(dead_code)]
const REG: &str = r"(?m)^([a-z]{5})(_[a-zA-Z]+)?\t[0-9]{4}(.*)$";

#[allow(dead_code)]
pub fn tidy_up_dictionary() {
    let file = File::open("././resources/dict.txt");
    match file {
        Err(e) => println!("Error opening dict file, {e}"),
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");
            sort_words(contents);
        }
    }
}

fn sort_words(contents: String) {
    let mut words = Vec::new();
    let mut empty = 0;
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let word = split.next().unwrap();
        let count = split.next().unwrap().parse::<u64>().unwrap();
        if count == 0 {
            empty += 1;
        }
        words.push((word, count));
    }
    println!("There are {} words with a count of 0", empty);
    write_to_dictionary(&mut words).expect("Failed to write to dictionary");
}

fn write_to_dictionary(words: &mut Vec<(&str, u64)>) -> Result<()> {
    let max = words.iter()
        .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
        .unwrap();

    println!("Most common word: {}, count: {:#?}", max.0, max.1);

    let mut oversized = false;
    let mut multiple = 1;
    if max.1 > u32::MAX as u64 {
        println!("highest count ({}) > u32::MAX ({})", max.1, u32::MAX);
        multiple = max.1 / u32::MAX as u64;
        println!("{} / {} = {}", max.1, u32::MAX, multiple);
        oversized = true;
    }

    let dictionary = File::create("././resources/dictionary.txt")
        .context("Failed to open file.")?;
    let mut buffer = BufWriter::new(dictionary);

    // Sort alphabetically (might want to sort by frequency, but we
    // can do that later, or make a separate dictionary for that)
    words.sort_by(|(w1, _), (w2, _)| w1.cmp(w2));
    let mut i = 0;
    for (word, mut count) in words {
        if oversized {
            // make sure count fits into a u32 if it's oversized, by
            // dividing by one more than the multiplier we calculated
            // earlier (which will have been rounded down, hence why we
            // use one more than the multiple), and adding 1 afterwards,
            // to ensure that any values that are 0 have at least 1 count
            count = (count / (multiple + 1)) + 1;
        }
        writeln!(buffer, "{word} {count}").context("Failed to write to buffer")?;
        i += 1;
        // flush buffer periodically
        if i % 500 == 0 {
            buffer.flush().context("Failed to flush buffer.")?;
        }
    }
    buffer.flush().context("Failed to flush buffer")?;
    Ok(())
}

#[allow(dead_code)]
pub fn decompress_files() {
    let dir = fs::read_dir("././resources").unwrap();
    for file in dir {
        if let Ok(file) = file {
            let name = file.file_name().into_string().unwrap();
            if name.ends_with(".gz") {
                let contents = decode_file(file.path());
                let mut path = file.path();
                path.set_extension("txt");
                println!("Path: {:?}", path);
                fs::write(path, contents).unwrap();
            }
        }
    }
}

fn decode_file(path: PathBuf) -> String {
    let file = File::open(path.clone()).unwrap();
    let mut decoder = GzDecoder::new(file);
    let mut string = String::new();
    let result = decoder.read_to_string(&mut string);
    if let Err(x) = result {
        println!("Error whilst decoding {:?}, {}", path, x);
    }
    string
}

#[allow(dead_code)]
pub fn create_dictionary() {
    let dir = fs::read_dir("././resources").expect("Failed to find directory '././resources'");
    let mut map = HashMap::from_iter(DICTIONARY.lines().map(|l| (l.to_string(), 0)));

    let mut parsed_files = Vec::new();
    for entry in dir {
        if let Ok(entry) = entry {
            let filename = entry.file_name().to_str().unwrap().to_owned();
            if filename.ends_with("00024.txt") {
                find_and_parse_matches(&entry.path(), &mut map);
                parsed_files.push(filename);
                write_map_to_file(&map, &parsed_files).expect("Failed to write to file");
            }
        }
    }
}

fn write_map_to_file(map: &HashMap<String, u64>, file_list: &Vec<String>) -> Result<()> {
    let dict = OpenOptions::new()
        .write(true)
        .open("././resources/dict.txt")
        .unwrap();
    let mut buffer = BufWriter::new(dict);
    let mut count = 0;
    for (w, c) in map {
        write!(buffer, "{w}")?;
        write!(buffer, " ")?;
        writeln!(buffer, "{c}")?;
        if count % 1000 == 0 {
            buffer.flush()?;
        }
        count += 1;
    }
    writeln!(buffer, "===========================")?;
    for file in file_list {
        writeln!(buffer, "{file}")?;
    }
    buffer.flush()?;
    Ok(())
}

fn find_and_parse_matches(path: &PathBuf, map: &mut HashMap<String, u64>) {
    println!("Parsing file: {:?}", path);

    let mut file = File::open(path).expect(&format!("Failed to open file {:?}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read file to string {:?}", path));

    let mut match_count = 0;
    let basic = Regex::new(REG).unwrap();
    for cap in basic.captures_iter(&contents) {
        let word = cap[1].to_lowercase().to_owned();

        // only parse rest of line if hashmap already has word in
        if let Some(value) = map.get_mut(&word) {
            // for each year entry, add occurence count to a total
            let cap_string = &cap[0];
            let parts = cap_string.split_whitespace();
            let mut total = 0;
            for m in parts.skip(1) {
                let trim_start = &m[5..];
                let end = trim_start.find(',').expect("Failed to find ','");
                let count = trim_start[..end]
                    .parse::<u64>()
                    .expect("Failed to parse to int");
                total += count;
            }

            *value += total;

            match_count += 1;
            if match_count % 100 == 0 {
                print!("\r\tmatches: {match_count}");
            }
        }
    }
    println!("");
    println!("\t======= finished parsing file, {match_count} matches ============");
    // buffer.flush().expect("Failed to flush buffer");
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read, path::Path};

    use super::REG;
    use regex::Regex;

    #[test]
    fn test_regex() {
        let regex = Regex::new(REG).unwrap();
        let no_capture = "Georgsmarien_NOUN	1901,4,2	1906,2,2	1907,1,1	1908,5,5\n";
        let capture2 = "EuroC	1878,1,1	1967,1,1	1968,1,1	1972,9,1\nBla bla bla";
        let capture3 = "Golem_NOUN	1875,1,1	1903,1,1	1915,1,1\nother stuff don't match";

        assert!(!regex.is_match(no_capture));
        assert!(regex.is_match(capture2));
        assert!(regex.is_match(capture3));

        let cap2 = regex.captures(capture2).unwrap();
        let match2 = cap2.iter().next().unwrap().unwrap();
        assert_eq!(match2.as_str(), "EuroC	1878,1,1	1967,1,1	1968,1,1	1972,9,1");

        let cap3 = regex.captures(capture3).unwrap();
        let match3 = cap3.iter().next().unwrap().unwrap();
        assert_eq!(match3.as_str(), "Golem_NOUN	1875,1,1	1903,1,1	1915,1,1");
    }

    #[test]
    fn test_capture() {
        let regex = Regex::new(REG).unwrap();
        let hay = "Golem_NOUN	1875,1,1	1903,1,1	1915,1,1\nother stuff don't match";

        assert!(regex.is_match(hay));

        let cap = regex.captures(hay).unwrap();
        let cap_string = &cap[0];
        assert_eq!(cap_string, "Golem_NOUN	1875,1,1	1903,1,1	1915,1,1");

        let word = &cap_string[0..5];
        let parts = cap_string.split_whitespace();
        let mut total = 0;
        for m in parts.skip(1) {
            let parts = m.split(',');
            let count = parts.skip(1).next().unwrap();
            total += count.parse::<i32>().unwrap();
        }
        assert_eq!(total, 3);
        let string = word.to_owned() + " " + &total.to_string();
        assert_eq!(string, "Golem 3")
    }

    #[test]
    fn capture_iters_test() {
        let regex = Regex::new(REG).expect("Failed to create regex");
        let hay = "First_NOUN\t1875,1,1\t1903,1,1\t1915,1,1\nother stuff don't match\nSecon_NOUN\t1875,1,1\t1903,1,1\t1915,1,1\nblahother_test\t2121 don't match\nThird\t1875,1,1\t1903,1,1\t1915,1,1";

        println!("{}", hay.to_string());

        let mut i = 0;
        for cap in regex.captures_iter(hay) {
            i += 1;
            let cap = &cap;
            println!("Capture {i}: {:?}", &cap[0]);
            println!("word: {}", &cap[1]);
        }
        assert_eq!(i, 3);
    }

    #[test]
    fn wtf_is_happening_with_regex() {
        let basic = Regex::new(r"(?m)^([a-z]{5})(_[a-zA-Z]+)?\t[0-9]{4}(.*)$").unwrap();
        let hay = include_str!("../../resources/test.txt");

        let mut i = 0;
        for cap in basic.captures_iter(hay) {
            i += 1;
            println!("Capture {i}, {:?}, first: {}", &cap[0], &cap[1]);
        }
        assert_eq!(i, 5);
    }

    #[test]
    fn open_file_test() {
        let mut contents = String::new();
        let path = Path::new("./resources/test.txt");
        let mut file = File::open(path).expect(&format!("Failed to open file {:?}", path));
        file.read_to_string(&mut contents)
            .expect(&format!("Failed to read file to string {:?}", path));

        let basic = Regex::new(r"(?m)^([a-z]{5})(_[a-zA-Z]+)?\t[0-9]{4}(.*)$").unwrap();
        let hay = include_str!("../../resources/test.txt");

        assert_eq!(hay, &contents);

        let mut hay_caps = 0;
        for cap in basic.captures_iter(hay) {
            hay_caps += 1;
            println!("Capture {hay_caps}, {:?}, first: {}", &cap[0], &cap[1]);
        }

        let mut cont_caps = 0;
        for cap in basic.captures_iter(&contents) {
            cont_caps += 1;
            println!("Capture {cont_caps}, {:?}, first: {}", &cap[0], &cap[1]);
        }
        assert_eq!(cont_caps, hay_caps);
    }
}
