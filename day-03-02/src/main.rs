use regex::Regex;

static INPUT: &'static str = include_str!("../../inputs/day-03/input.txt");

fn extract_instructions(line: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    re.find_iter(line).map(|inst| inst.as_str()).collect::<Vec<&str>>()
}

fn exec_instruction(inst: &str) -> i32 {
    println!("Exec Instruction: {}", inst);
    let re = Regex::new(r"([0-9]{1,3}),([0-9]{1,3})").unwrap();
    return if let Some(caps) = re.captures(inst) {
        let left = &caps[1].parse::<i32>().expect("expected a number");
        let right = &caps[2].parse::<i32>().expect("expected a number");
        left * right
    } else {
        0
    }
}

fn main() {
    let mut result = 0;
    let mut math_on = true;
    INPUT
        .lines()
        .for_each(|line| {
            let instructions = extract_instructions(line);
            for inst in instructions {
                if inst == "do()" {
                    println!("Math ON");
                    math_on = true;
                } else if inst == "don't()" {
                    println!("Math OFF");
                    math_on = false;
                } else if math_on {
                    result += exec_instruction(inst);
                } else {
                    println!("SKIPPED: {}", inst);
                }
            }
        });

    println!("Result is {}", result);
}
