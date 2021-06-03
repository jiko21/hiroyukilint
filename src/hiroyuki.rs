mod miss {
    #[derive(Debug, Eq, PartialEq)]
    pub enum MissType {
        Impression,
        Lie,
    }

    pub fn get_miss_type(reason: &str) -> MissType {
        match reason {
            "明らかに" => MissType::Impression,
            _ => MissType::Lie,
        }
    }

    pub fn get_reason(miss_type: MissType) -> &'static str {
        match miss_type {
            MissType::Impression => {
                &"明らかにって、それってあなたの感想ですよね?なんかデータとかあるんすか？"
            }
            _ => &"なんだろう、ウソつくのやめてもらってもいいっすか？",
        }
    }
}

mod check {
    use super::miss::{get_miss_type, MissType};
    #[derive(Debug, Eq, PartialEq)]
    pub struct CheckPoint {
        pub line: usize,
        pub start: usize,
        pub end: usize,
        pub miss_type: MissType,
    }

    impl CheckPoint {
        pub fn new(line: usize, start: usize, end: usize, reason: &str) -> CheckPoint {
            CheckPoint {
                line,
                start,
                end,
                miss_type: get_miss_type(reason),
            }
        }
    }
}

pub mod checker {

    extern crate regex;

    use counted_array::counted_array;
    use regex::Regex;
    use std::io::{stdout, BufRead, BufWriter, Write};

    use crate::hiroyuki::miss::get_reason;

    use super::check::CheckPoint;

    counted_array!(const WORD_LISTS: [&str; _] = [
        "絶対に",
        "確実に",
        "必ず",
        "明らかに",
    ]);

    const BLANK: &str = " ";
    const CURSOR: &str = "^";

    fn check(line: usize, text: &String) -> Vec<CheckPoint> {
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

    pub struct Checker<B: BufRead> {
        reader: B,
    }

    impl<B> Checker<B>
    where
        B: BufRead,
    {
        pub fn new(reader: B) -> Checker<B> {
            Checker::<B> { reader }
        }
        pub fn print(&mut self) {
            let out = stdout();
            let mut out = BufWriter::new(out.lock());
            let mut code_line = 0;
            let mut error_count = 0;
            for line in self.reader.by_ref().lines() {
                match line {
                    Ok(line) => {
                        let points = check(code_line, &line);
                        error_count += points.len();
                        for point in points {
                            writeln!(out, "{}\n", text_line(code_line, &line, point)).unwrap();
                        }
                        code_line += 1;
                    }
                    Err(_) => {
                        eprintln!("Cannot read line @ {}", code_line);
                        return;
                    }
                };
            }
            writeln!(out, "found {} errors", error_count).unwrap();
        }
    }

    fn text_line(line: usize, text: &String, check_point: CheckPoint) -> String {
        let _line = format!("line {}: ", line);
        let first_line = format!("{}{}", _line, text);
        let mut point_cursor = "".to_string();
        let mut i = 0;
        let mut cursor_at = 0;
        while i < check_point.start {
            if let Some(_) = text.get(i..(i + 1)) {
                i += 1;
                cursor_at += 1;
            } else {
                i += 3;
                cursor_at += 2;
            }
        }
        point_cursor.push_str(&BLANK.to_string().repeat(_line.len() + cursor_at));
        cursor_at = 0;
        while i < check_point.end {
            if let Some(_) = text.get(i..(i + 1)) {
                i += 1;
                cursor_at += 1;
            } else {
                i += 3;
                cursor_at += 2;
            }
        }
        point_cursor.push_str(&CURSOR.to_string().repeat(cursor_at));
        format!(
            "{}\n{}\n{}",
            first_line,
            point_cursor,
            get_reason(check_point.miss_type)
        )
    }

    #[cfg(test)]
    mod test {
        use crate::hiroyuki::{check::CheckPoint, checker::check, miss::MissType};

        #[test]
        fn test_check() {
            assert_eq!(
                check(11, &"絶対にa正しい明らかに".to_string()),
                vec![
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
                ]
            );
        }
    }
}
