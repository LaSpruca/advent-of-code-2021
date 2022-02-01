const INPUT: &str = include_str!("input.txt");

fn main() {
    let readings = INPUT
        .split("\n")
        .map(|f| f.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let size = readings.len();

    let mut windows = vec![];

    for index in 0..size {
        if (index + 2) >= size {
            break;
        }

        windows.push(readings[index] + readings[index + 1] + readings[index + 2])
    }

    let result = windows
        .iter()
        .fold((0, -1), |acc, x| if x.to_owned() > acc.0 { (x.to_owned(), acc.1 + 1) } else { (x.to_owned(), acc.1) });

    println!("{}", result.1);
}
