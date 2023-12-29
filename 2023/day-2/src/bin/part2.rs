use std::{collections::HashMap, fs};

fn parse_game(game: &str) -> (u32, &str) {
    let mut parts = game.splitn(2, ' ');

    let count = parts
        .next()
        .expect("count is missing")
        .parse::<u32>()
        .expect("couldnt convert color count to u32");

    let color = parts.next().expect("color is missing");

    (count, color)
}

fn process_line(line: &str) -> u32 {
    let parts = line.splitn(2, ':');

    let mut max_colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    // need to find max of each color in each game_id
    // then multiply the values together
    // then sum
    parts
        .last()
        .expect("games is missing")
        .split(';')
        .flat_map(|s| s.split(',').map(|s| s.trim()))
        .for_each(|s| {
            let (count, color) = parse_game(s);

            let max_count = std::cmp::max(count, *max_colors.get(color).expect("yo"));

            max_colors.insert(color, max_count);
        });

    max_colors
        .into_values()
        .reduce(|acc, e| acc * e)
        .expect("multiply max_colors failed")
}

fn cube_conundrum(content: &str) -> u32 {
    content.lines().map(process_line).sum::<u32>()
}

fn main() {
    let content = fs::read_to_string("games.txt").expect("file should exist");

    let result = cube_conundrum(content.trim());

    dbg!(result);
}

#[cfg(test)]
mod tests {
    use crate::{cube_conundrum, parse_game};

    #[test]
    fn parse_game_works_as_expected() {
        let game = "3 blue";

        let (count, color) = parse_game(game);

        assert_eq!(count, 3);
        assert_eq!(color, "blue");
    }

    #[test]
    fn it_returns_correct_result_for_single_game() {
        let games = "
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .trim();

        let result = cube_conundrum(games);

        assert_eq!(result, 48);
    }

    #[test]
    fn it_returns_correct_result() {
        let games = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim();

        let result = cube_conundrum(games);

        assert_eq!(result, 2286);
    }
}
