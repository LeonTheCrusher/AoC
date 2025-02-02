use std::fs::read_to_string;

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUMS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn main() {
    let content: String = read_to_string("coordinates.txt").unwrap();

    let mut sum: i32 = 0;

    // easy
    for line in content.lines() {
        sum += convert(easy(&line.to_string()));
    }
    println!("Easy: {}", sum);

    sum = 0;
    // hard
    for line in content.lines() {
        sum += convert(hard(&line.to_string()));
    }
    println!("Hard: {}", sum);
}

fn easy(string: &String) -> String {
    let mut new = String::new();
    for c in string.chars() {
        if c.is_ascii_digit() {
            new.push(c);
        }
    }
    new
}

fn hard(string: &String) -> String {
    let len = string.len();
    let mut new = String::new();
    for i in 0..len {
        if string.chars().nth(i).unwrap().is_ascii_digit() {
            new.push(string.chars().nth(i).unwrap());
            continue;
        }
        for j in 3..=5 {
            if i + j <= len {
                for k in 0..WORDS.len() {
                    if &string[i..i + j] == WORDS[k] {
                        new.push_str(NUMS[k]);
                    }
                }
            } else {
                break;
            }
        }
    }
    new
}

fn convert(input: String) -> i32 {
    let mut output = String::new();
    let len = input.len();
    let first = &input[0..1];
    let last;
    if len > 1 {
        last = &input[len - 1..len];
    } else {
        last = &input[0..1];
    }

    output.push_str(first);
    output.push_str(last);
    output.parse().unwrap()
}
