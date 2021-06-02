extern crate regex;

use regex::Regex;
use counted_array::counted_array;

counted_array!(const WORD_LISTS: [&str; _] = [
    "絶対に",
    "確実に",
    "必ず",
    "明らかに",
]);

const BLANK: &str = " ";
const CURSOR: &str = "^";

#[derive(Debug, Eq, PartialEq)]
pub enum MissType {
    Impression,
    Lie,
}

fn get_miss_type(reason: &str) -> MissType {
    match reason {
        "明らかに" => MissType::Impression,
        _ => MissType::Lie,
    }
}

fn get_reason(miss_type: MissType) -> &'static str {
    match miss_type {
        MissType::Impression => &"明らかにって、それってあなたの感想ですよね?なんかデータとかあるんすか？",
        _ => &"なんだろう、ウソつくのやめてもらってもいいっすか？",
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CheckPoint {
    line: usize,
    start: usize,
    end: usize,
    miss_type: MissType,
}

impl CheckPoint {
    fn new(line: usize, start: usize, end: usize, reason: &str) -> CheckPoint {
        CheckPoint{
            line,
            start,
            end,
            miss_type: get_miss_type(reason),
        }
    }
}

pub fn check(line: usize, text: &String) -> Vec<CheckPoint> {
    let mut point_vec: Vec<CheckPoint> = Vec::new();
    for word in WORD_LISTS.iter() {
        let re = Regex::new(word).unwrap();
        for matches in re.find_iter(&text) {
            let start = matches.start();
            let end = matches.end();
            let check_point = CheckPoint::new(line, start, end, word);
            point_vec.push(check_point);
        }
    }
    point_vec
}

pub fn print(line: usize, text: &String, check_point: CheckPoint) {
    let _line = format!("line {}: ", line);
    println!("{}{}", _line, text);
    for _ in 0.._line.len() {
        print!(" ");
    }
    let mut point_cursor = "".to_string();
    let mut i = 0;
    let mut cursor_at = 0;
    while i < check_point.start {
        if let Some(_) = text.get(i..(i+1)) {
            i += 1;
            cursor_at += 1;
        } else {
            i += 3;
            cursor_at += 2;
        }
    };
    point_cursor.push_str(&BLANK.to_string().repeat(cursor_at));
    cursor_at = 0;
    while i < check_point.end {
        if let Some(_) = text.get(i..(i+1)) {
            i += 1;
            cursor_at += 1;
        } else {
            i += 3;
            cursor_at += 2;
        }
    };
    point_cursor.push_str(&CURSOR.to_string().repeat(cursor_at));
    println!("{}", point_cursor);
    println!("{}\n", get_reason(check_point.miss_type));
}

#[cfg(test)]
mod tests {
    use crate::hiroyuki::CheckPoint;
    use crate::hiroyuki::MissType;
    use crate::hiroyuki::check;

    #[test]
    fn test_check() {
        assert_eq!(check(11, &"絶対にa正しい明らかに".to_string()), vec![
            CheckPoint {
                line: 11,
                start: 0,
                end: 9,
                miss_type: MissType::Lie,
            },
            CheckPoint {
                line: 11,
                start: 19,
                end: 31,
                miss_type: MissType::Impression,
            },
        ]);
    }
}
