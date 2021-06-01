extern crate regex;

use regex::Regex;
use counted_array::counted_array;

counted_array!(const WORD_LISTS: [&str; _] = [
    "絶対に",
    "確実に",
    "必ず"
]);

pub fn check(text: String) -> bool {
    for word in WORD_LISTS.iter() {
        let re = Regex::new(word).unwrap();
        println!("{}", text);
        for matches in re.find_iter(&text) {
            for i in 0..text.len() / 3 {
                if i >=  matches.start() / 3 && i < matches.end() / 3 {
                    print!("^");
                } else {
                    print!(" ");
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::hiroyuki::check;

    #[test]
    fn test_check() {
        assert_eq!(check("絶対に正しい絶対に、確実に失敗する".to_string()), true);
    }
}
