const INPUT: &str = include_str!("one-input.txt");

fn main() {
    let result = INPUT
        .split("\n")
        .map(|f| f.trim().parse::<i32>().unwrap())
        .fold((0, -1), |acc, x| if x > acc.0 { (x, acc.1 + 1) } else { (x, acc.1) });
    println!("{}", result.1);
}
