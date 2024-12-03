use regex::Regex;

static INPUT: &'static str = include_str!("../../inputs/day-03/input.txt");

fn extract_instructions(line: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
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

    INPUT
        .lines()
        .for_each(|line| {
            let instructions = extract_instructions(line);
            for inst in instructions {
                result += exec_instruction(inst);
            }
        });

    println!("Result is {}", result);
}
