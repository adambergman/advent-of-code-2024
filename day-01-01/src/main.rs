static INPUT: &'static str = include_str!("../../inputs/day-01/input.txt");

fn main() {
    let lines = INPUT.split("\n").collect::<Vec<&str>>();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in lines.iter() {
        let pair = line.split_whitespace().collect::<Vec<&str>>();
        let value_a = pair[0].parse::<i32>().unwrap();
        let value_b = pair[1].parse::<i32>().unwrap();
        a.push(value_a);
        b.push(value_b);
    }
    a.sort_by(|x, y| x.cmp(y));
    b.sort_by(|x, y| x.cmp(y));
    let mut distance = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        distance += (a - b).abs();
        println!("{} - {} = {} === {}", a, b, (a - b).abs(), distance);
    }
    println!("Total distance: {}", distance);
}
