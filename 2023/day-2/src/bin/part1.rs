use std::fs;

struct GameConfig {
    green: u32,
    red: u32,
    blue: u32,
}

static GAME_CONFIG: GameConfig = GameConfig {
    green: 13,
    red: 12,
    blue: 14,
};

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

// This function should return a Option? so that can we can
// filter_map in process function
fn process_line(line: &str) -> u32 {
    let mut parts = line.splitn(2, ':');

    let game_id = parts
        .next()
        .expect("id part missing")
        .split(' ')
        .last()
        .expect("id is missing")
        .parse::<u32>()
        .expect("couldnt parse id to number");

    // let mut green = 0;
    // let mut blue = 0;
    // let mut red = 0;

    // keep a sum of red, green, blue
    // run through each game and update corresponding colors sum
    // if sum is over game_config then game is invalid and
    // dont add the game id to the sum
    let games = parts
        .next()
        .expect("games is missing")
        .split(';')
        .flat_map(|s| s.split(',').map(|s| s.trim()))
        .collect::<Vec<_>>();

    let valid = games
        .iter()
        .map(|s| {
            let (count, color) = parse_game(s);

            match color {
                "green" => {
                    // dbg!(count, color, GAME_CONFIG.green);

                    count > GAME_CONFIG.green
                }
                "blue" => count > GAME_CONFIG.blue,
                "red" => count > GAME_CONFIG.red,
                _ => false,
            }
        })
        .all(|v| !v);

    if valid {
        game_id
    } else {
        0
    }
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

        assert_eq!(result, 1);
    }

    #[test]
    fn it_returns_correct_result_for_invalid_game() {
        let games = "
Game 1: 15 blue, 12 red; 1 red, 2 green, 6 blue; 2 green"
            .trim();

        let result = cube_conundrum(games);

        assert_eq!(result, 0);
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

        assert_eq!(result, 8);
    }
}
