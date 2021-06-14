mod miss {
    #[derive(Debug, Eq, PartialEq)]
    pub enum MissType {
        Impression,
        Lie,
        UnknownWord { word: String },
    }

    pub fn get_miss_type(reason: &str) -> MissType {
        match reason {
            "明らかに" => MissType::Impression,
            "絶対に" | "確実に" => MissType::Lie,
            _ => MissType::UnknownWord {
                word: reason.to_string(),
            },
        }
    }

    pub fn get_reason(miss_type: MissType) -> String {
        match miss_type {
            MissType::Impression => {
                "明らかにって、それってあなたの感想ですよね?なんかデータとかあるんすか？"
                    .to_string()
            }
            MissType::Lie => "なんだろう、ウソつくのやめてもらってもいいっすか？".to_string(),
            MissType::UnknownWord { word } => format!("なんすか？{}って", word),
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

    use regex::Regex;
    use std::io::{stdout, BufRead, BufWriter, Write};

    use crate::hiroyuki::miss::get_reason;

    use super::check::CheckPoint;

    const BLANK: &str = " ";
    const CURSOR: &str = "^";

    fn check(line: usize, text: &String, blacklists: &Vec<String>) -> Vec<CheckPoint> {
        let mut point_vec: Vec<CheckPoint> = Vec::new();
        for word in blacklists.iter() {
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
        blacklists: Vec<String>,
    }

    impl<B> Checker<B>
    where
        B: BufRead,
    {
        pub fn new(reader: B, lists: &mut Vec<String>) -> Checker<B> {
            let mut blacklists = vec![
                "絶対に".to_string(),
                "確実に".to_string(),
                "必ず".to_string(),
                "明らかに".to_string(),
            ];
            blacklists.append(lists);
            Checker::<B> { reader, blacklists }
        }
        pub fn print(&mut self) {
            let out = stdout();
            let mut out = BufWriter::new(out.lock());
            let mut code_line = 0;
            let mut error_count = 0;
            for line in self.reader.by_ref().lines() {
                match line {
                    Ok(line) => {
                        let points = check(code_line, &line, &self.blacklists);
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
                check(
                    11,
                    &"絶対にa正しい明らかに、しゃぞーです".to_string(),
                    &vec![
                        "絶対に".to_string(),
                        "明らかに".to_string(),
                        "しゃぞー".to_string()
                    ]
                ),
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
                    CheckPoint {
                        line: 11,
                        start: 34,
                        end: 46,
                        miss_type: MissType::UnknownWord {
                            word: "しゃぞー".to_string(),
                        },
                    },
                ]
            );
        }
    }
}
