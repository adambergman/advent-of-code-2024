static INPUT: &'static str = include_str!("../../inputs/day-01/input.txt");

fn main() {
    let lines = INPUT.split("\n").collect::<Vec<&str>>();
    let mut a:Vec<i32> = vec![0; 0];
    let mut b:Vec<i32> = vec![0; 0];
    for line in lines.iter() {
        let pair = line.split_whitespace().collect::<Vec<&str>>();
        let value_a = pair[0].parse::<i32>().unwrap();
        let value_b = pair[1].parse::<i32>().unwrap();
        a.push(value_a);
        b.push(value_b);
    }
    let mut score = 0;
    for loc in a.iter() {
        let count:i32 = b.iter().filter(|b| *b == loc).count() as i32;
        let calc:i32 = count * loc;
        println!("Score: {} + {} ({} * {}) = {}", score, calc, loc, count, score + calc);
        score += calc;
    }
    println!("Total score: {}", score);
}
