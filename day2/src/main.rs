use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "game.pest"]
struct GameParser;

fn part1(file: pest::iterators::Pair<Rule>) {
    let mut sum_of_game_id = 0;
    for game in file.into_inner() {
        let mut game_valid = true;
        let mut inner_game = game.into_inner();
        let game_id = inner_game.next().unwrap().as_str().parse::<i32>().unwrap();
        let game_content = inner_game.next().unwrap();

        for game_set in game_content.into_inner() {
            for cube_data in game_set.into_inner() {
                let mut inner_cube = cube_data.into_inner();
                let cube_count_str = inner_cube.next().unwrap().as_str().trim();
                let cube_count = cube_count_str.parse::<i32>().unwrap();
                let cube_color = inner_cube.next().unwrap().as_str();
                let max_valid_count = match cube_color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => unreachable!(),
                };
                if cube_count > max_valid_count {
                    game_valid = false;
                }
            }
        }
        if game_valid {
            sum_of_game_id += game_id;
        }
    }
    println!("Part 1");
    println!("Sum of valid game id: {}", sum_of_game_id);
}

fn part2(file: pest::iterators::Pair<Rule>) {
    let mut sum_of_powers = 0;
    for game in file.into_inner() {
        let mut inner_game = game.into_inner();
        let _game_id = inner_game.next().unwrap().as_str().parse::<i32>().unwrap();
        let game_content = inner_game.next().unwrap();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for game_set in game_content.into_inner() {
            for cube_data in game_set.into_inner() {
                let mut inner_cube = cube_data.into_inner();
                let cube_count_str = inner_cube.next().unwrap().as_str().trim();
                let cube_count = cube_count_str.parse::<i32>().unwrap();
                let cube_color = inner_cube.next().unwrap().as_str();
                match cube_color {
                    "red" => {
                        if cube_count > min_red {
                            min_red = cube_count;
                        }
                    }
                    "green" => {
                        if cube_count > min_green {
                            min_green = cube_count;
                        }
                    }
                    "blue" => {
                        if cube_count > min_blue {
                            min_blue = cube_count;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        sum_of_powers += min_red * min_green * min_blue;
    }
    println!("Part 2");
    println!("Sum of powers: {}", sum_of_powers);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let file = GameParser::parse(Rule::file, &input)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();
    // part1(file);
    part2(file);
}
