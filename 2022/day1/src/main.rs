fn main() {
    let input = aoc_lib::get_input(1, |s| String::from(s)).unwrap();
    let calories_list = parse_calories_list(&input);

    println!("Part 1: {}", part_1(&calories_list));
    println!("Part 2: {}", part_2(&calories_list));
}

fn parse_calories_list(input: &[String]) -> Vec<u32> {
    let mut result = Vec::new();
    let mut tmp = 0;

    for line in input {
        if line.is_empty() {
            result.push(tmp);
            tmp = 0;
        } else {
            tmp += line.parse::<u32>().unwrap();
        }
    }

    result
}

fn part_1(calories_list: &[u32]) -> u32 {
    *calories_list.iter().max().unwrap_or(&0)
}

fn part_2(calories: &[u32]) -> u32 {
    let mut calories_list = Vec::from(calories);
    calories_list.sort_by(|a, b| b.cmp(a));

    calories_list[0] + calories_list[1] + calories_list[2]
}