static INPUT: &'static str = include_str!("../../inputs/day-02/input.txt");

fn main() {
   let mut safe_count: i32 = 0;
   let mut unsafe_count: i32 = 0;
    let mut index = 0;
   INPUT
    .lines()
       .for_each(|line| {
           index += 1;
         // Parse each line in to a values array
           let values = line
             .split_whitespace()
             .collect::<Vec<&str>>()
             .into_iter()
             .map(|val| val.parse::<i32>());

         // Check the values for safe/unsafe
           let mut trend = 0;
           let mut last = 0;
           let mut start: bool = true;
           let mut is_safe = true;
           for val in values {
               let value = val.unwrap();
               if start {
                   start = false;
                   last = value;
                   continue;
               }
               if trend == 0 {
                   trend = match last > value { true => 1, false => -1 };
               }
               let result = last - value;
               if (trend == -1 && result > 0) || (trend == 1 && result < 0) {
                   is_safe = false;
               } else if result.abs() > 3 || result.abs() < 1 {
                   is_safe = false;
               }
               last = value;
           }
           if is_safe {
               safe_count += 1;
           } else {
               unsafe_count += 1;
           }
           // println!("Line {}: {}; Total: {}", index, is_safe, safe_count);
       });
    println!("safe count: {}", safe_count);
}
