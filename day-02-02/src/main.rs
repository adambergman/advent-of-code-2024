static INPUT: &'static str = include_str!("../../inputs/day-02/input.txt");

fn make_values(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|val| val.parse::<i32>().expect("Invalid integer"))
        .collect::<Vec<i32>>()
}

fn check_vec(values: &Vec<i32>) -> bool {
    if values.len() < 2usize {
        return false
    }
    let mut safe = true;
    let mut trend = 0;
    let mut prev: (i32, bool) = (values[0], true); // Previous value and true if it's the first one
    for val in values {
        if prev.1 {
            prev.1 = false;
            continue;
        }
        if trend == 0 {
            trend = match prev.0 > *val { true => 1, false => -1 };
        }
        let result = prev.0 - val;
        if (trend == -1 && result > 0) || (trend == 1 && result < 0) {
            safe = false;
        } else if result.abs() > 3 || result.abs() < 1 {
            safe = false;
        }
        prev.0 = *val;
    }
    safe
}

fn is_safe(line: &str) -> bool {
    let values = make_values(line);
    let safe = check_vec(&values);
    if safe {
        println!("  safe [F]: {}", line);
        return true;
    }
    for idx in 0..values.len() {
        let mut split = values.clone();
        split.remove(idx);
        let safe = check_vec(&split);
        if safe {
            println!("  safe [{}]: {}", idx, line);
            return true;
        }
    }
    println!("unsafe [E]: {}", line);
    false
}

fn main() {
    let mut safe_count: i32 = 0;
    let mut unsafe_count: i32 = 0;
    let mut index = 0;
    INPUT
        .lines()
        .for_each(|line| {
            index += 1;
            // Parse each line in to a values array
            if is_safe(line) {
                safe_count += 1;
            } else {
                unsafe_count += 1;
            }
            // println!("Line {}: {}; Total: {}", index, is_safe, safe_count);
        });
    println!("safe count: {}", safe_count);
    println!("unsafe count: {}", unsafe_count);
}
