const MIN_RANGE: i32 = 152085;
const MAX_RANGE: i32 = 670283;

fn has_repeat_char(code_str: &String) -> bool {
    let mut code_iter = code_str.chars().peekable();
    while let Some(c) = code_iter.next() {
        if Some(&c) == code_iter.peek() {
            return true;
        }
    }
    false
}

fn has_single_repeat_char(code_str: &String) -> bool {
    let mut code_iter = code_str.chars().peekable();
    while let Some(c) = code_iter.next() {
        if Some(&c) == code_iter.peek() {
            match code_iter.next() {
                Some(_) => {
                    if Some(&c) != code_iter.peek() {
                        return true;
                    } else {
                        while let Some(n) = code_iter.next() {
                            if Some(&n) != code_iter.peek() {
                                break;
                            }
                        }
                    }
                }
                None => {
                    return true;
                }
            }
        }
    }
    false
}

fn digits_increase(code_str: &String) -> bool {
    let mut code_iter = code_str.chars().peekable();
    while let Some(c) = code_iter.next() {
        let code_num: u32 = c.to_digit(10).unwrap();
        let peek_char = code_iter.peek();
        if peek_char.is_some() {
            let peek_num: u32 = peek_char.unwrap().to_digit(10).unwrap();
            if code_num > peek_num {
                return false;
            }
        }
    }
    true
}

fn main() {
    let count: usize = (MIN_RANGE..=MAX_RANGE)
        .map(|n| n.to_string())
        .filter(has_repeat_char)
        .filter(digits_increase)
        .count();
    println!("Part 1: {:?}", count);
    let count2: usize = (MIN_RANGE..=MAX_RANGE)
        .map(|n| n.to_string())
        .filter(has_single_repeat_char)
        .filter(digits_increase)
        .count();
    println!("Part 2: {:?}", count2);
}
