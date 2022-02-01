const INPUT: &str = include_str!("input.txt");

fn main() {
    let result = INPUT
        .split("\n")
        .map(|instruction| {
            let parts = instruction.trim().split(" ").collect::<Vec<_>>();
            let inst = parts[0];
            let unit = parts[1].parse::<i32>().unwrap();

            match inst {
                "forward" => Some((0, unit)),
                "up" => Some((1, -unit)),
                "down" => Some((1, unit)),
                _ => None
            }
        })
        .filter_map(|f| f)
        .fold((0, 0, 0),
              |(aim, x, y), (index, units)| if index == 0 { (aim, x + units, y + units * aim) } else { (aim + units, x, y) });
    println!("{}", result.1 * result.2);
}
