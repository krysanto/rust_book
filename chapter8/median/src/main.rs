use std::collections::HashMap;

fn main() {
    let mut numbers = vec![7,5,1,2,3,4,6,4,4,7,7,7,7,7,7];
    numbers.sort();
    println!("{:?}", numbers);

    let mut number_map = HashMap::new();

    for number in &numbers{
        let count = number_map.entry(number).or_insert(1);
        *count += 1;
    }

    let mut highest_number: (i32, i32) = (-1, -1);
    for number in number_map.iter() {
        if number.1 > &highest_number.1 {
            highest_number = (**number.0, *number.1);
        }
    }

    println!("Median: {}", numbers[numbers.len()/2]);
    println!("Mode: {}", highest_number.0);
}
