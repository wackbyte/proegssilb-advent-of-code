use std::cmp::max;

use aoc_zen_runner_macros::{aoc, generator, solution, solver};

#[aoc(2023, day2)]
pub mod solutions {
    use super::*;

    // #[generator(gen)]
    // pub fn input_generator(input: &str) -> Vec<Vec<Grab>> {

    // }

    // ----------------------- Part 1 -----------------------

    #[derive(Debug)]
    pub enum Seeking {
        Start,
        Count,
        Color,
        Delimeter,
        Newline,
    }

    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> u32 {
        let mut tally = 0;
        const RED_MAX: u8 = 12;
        const GREEN_MAX: u8 = 13;
        const BLUE_MAX: u8 = 14;
        let mut count_val = 0;
        let mut check_val = 0;
        let mut state = Seeking::Start;
        let mut game_num = 0;

        for c in input.as_bytes() {
            // println!("Current char: {} (State: count {}, check {}, state {:?}, game # {}", *c as char, count_val, check_val, state, game_num);
            match state {
                Seeking::Start => {
                    // println!("Reading start: char: {}, game num: {}", *c as char, game_num);
                    match c {
                        b':' => {
                            state = Seeking::Count;
                        }
                        b'0'..=b'9' => game_num = game_num * 10 + (c - b'0'),
                        _ => {
                            continue;
                        }
                    }
                }
                Seeking::Count => {
                    if c == &b' ' {
                        if count_val > 0 {
                            state = Seeking::Color;
                        }
                    } else {
                        count_val = count_val * 10 + (c - b'0');
                    }
                }
                Seeking::Color => {
                    match c {
                        b'r' => {
                            check_val = RED_MAX;
                        }
                        b'g' => {
                            check_val = GREEN_MAX;
                        }
                        b'b' => {
                            check_val = BLUE_MAX;
                        }
                        b' ' => {
                            continue;
                        }
                        b',' => {
                            unreachable!("comma while seeking a color");
                        }
                        b';' => {
                            unreachable!("semicolon while seeking a color");
                        }
                        _ => {
                            unreachable!("'{}' while seeking a color", *c as char);
                        }
                    }

                    if count_val > check_val {
                        // Bad round.
                        state = Seeking::Newline;
                    } else {
                        state = Seeking::Delimeter;
                    }
                }
                Seeking::Delimeter => match c {
                    b',' => {
                        count_val = 0;
                        check_val = 0;
                        state = Seeking::Count;
                    }
                    b';' => {
                        count_val = 0;
                        check_val = 0;
                        state = Seeking::Count;
                    }
                    b'\n' => {
                        // println!("Good game! #{}", game_num);
                        tally += game_num as u32;
                        count_val = 0;
                        check_val = 0;
                        game_num = 0;
                        state = Seeking::Start;
                    }
                    _ => {
                        continue;
                    }
                },
                Seeking::Newline => {
                    if c == &b'\n' {
                        state = Seeking::Start;
                        count_val = 0;
                        check_val = 0;
                        game_num = 0;
                    }
                }
            }
        }

        tally
    }

    // ---- Chumsky Draft ----
    pub struct Game(u32, Grab);

    #[derive(Default)]
    pub struct Grab {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Grab {
        fn set_component(self, c: Component) -> Grab {
            match c {
                Component(n, Color::Red) => Grab { red: n, green: self.green, blue: self.blue },
                Component(n, Color::Green) => Grab { red: self.red, green: n, blue: self.blue },
                Component(n, Color::Blue) => Grab { red: self.red, green: self.green, blue: n },
            }            
        }
    }

    pub struct Component(u8, Color);

    #[derive(Clone)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    impl Color {
        fn from_str(color: &str) -> Color {
            match color {
                "red" | "r" => Color::Red,
                "green" | "g" => Color::Green,
                "blue" | "b" => Color::Blue,
                _ => unreachable!(),
            }
        }
    }

    pub fn parser<'a>() -> impl chumsky::Parser<char, Vec<Game>, Error = chumsky::error::Simple<char>> {
        use chumsky::prelude::*;

        let color = text::keyword("red").to(Color::Red)
            .or(text::keyword("green").to(Color::Green))
            .or(text::keyword("blue").to(Color::Blue))
            .padded();

        let component = text::int(10).map(|x: String| x.parse::<u8>().unwrap()).padded().then(color).map(|(n, c)| Component(n, c));

        let grab = component
            .padded()
            .separated_by(just(','))
            .map(|cs| (Grab::default(), cs))
            .foldl(|g, c| g.set_component(c));

        let grabs = grab.clone()
            .padded()
            .then(just(';').then(grab.padded()).repeated())
            .foldl(|a: Grab, (_, b)| Grab {red: max(a.red, b.red), green: max(a.green, b.green), blue: max(a.blue, b.blue)});

        let game = text::keyword("Game")
            .padded()
            .then(text::int(10).padded().map(|x: String| x.parse::<u8>().unwrap()))
            .then(just(':').padded())
            .then(grabs)
            .map(|(((_, n), _), gs)| Game(n as u32, gs));

        game.repeated()
    }

    pub fn parser1<'a>() -> impl chumsky1::Parser<
        'a,
        &'a str,
        Vec<Game>,
        chumsky1::extra::Err<chumsky1::error::Simple<'a, char>>,
    > {
        use chumsky1::prelude::*;

        let color = text::keyword("red")
            .to(Color::Red)
            .or(text::keyword("green").to(Color::Green))
            .or(text::keyword("blue").to(Color::Blue))
            .padded();

        let component = text::int(10)
            .from_str::<u8>()
            .unwrapped()
            .padded()
            .then(color)
            .map(|(n, c)| Component(n, c));

        let grab = component
            .clone()
            .padded()
            .map(|cs| Grab::default().set_component(cs))
            .foldl(
                just(',').ignore_then(component.padded()).repeated(),
                |g, c| g.set_component(c),
            );

        let grabs =
            grab.clone()
                .padded()
                .foldl(just(';').ignore_then(grab.padded()).repeated(), |a, b| {
                    Grab {
                        red: max(a.red, b.red),
                        green: max(a.green, b.green),
                        blue: max(a.blue, b.blue),
                    }
                });

        let game = text::keyword("Game")
            .padded()
            .ignore_then(text::int(10).padded().from_str::<u8>().unwrapped())
            .then_ignore(just(':').padded())
            .then(grabs)
            .map(|(n, gs)| Game(n as u32, gs));

        game.repeated().collect()
    }

    #[generator(chumsky)]
    pub fn chumsky_parser(input: &str) -> Vec<Game> {
        use chumsky::prelude::*;

        parser().parse(input).unwrap()
    }

    #[generator(chumsky1)]
    pub fn chumsky1_parser(input: &str) -> Vec<Game> {
        use chumsky1::prelude::*;

        parser1().parse(input).unwrap()
    }

    #[solver(part1, chumsky)]
    pub fn part1_chumsky(input: Vec<Game>) -> u32 {
        input
            .into_iter()
            .filter_map(|Game(n, grb)| {
                if grb.red <= 12 && grb.green <= 13 && grb.blue <= 14 {
                    Some(n)
                } else {
                    None
                }
            })
            .sum()
    }

    #[solver(part1, chumsky1)]
    pub fn part1_chumsky1(input: Vec<Game>) -> u32 {
        input
            .into_iter()
            .filter_map(|Game(n, grb)| {
                if grb.red <= 12 && grb.green <= 13 && grb.blue <= 14 {
                    Some(n)
                } else {
                    None
                }
            })
            .sum()
    }

    // ----------------------- Part 2 -----------------------
    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> u32 {
        let mut tally = 0;

        let mut count_val = 0;
        let mut state = Seeking::Start;
        let mut game_num = 0;

        let mut min_red = 0u32;
        let mut min_green = 0u32;
        let mut min_blue = 0u32;

        for c in input.as_bytes() {
            //println!("Current char: {} (State: count {}, state {:?}, r/g/b: {}/{}/{}", *c as char, count_val, state, min_red, min_green, min_blue);
            match state {
                Seeking::Start => {
                    // println!("Reading start: char: {}, game num: {}", *c as char, game_num);
                    match c {
                        b':' => {
                            state = Seeking::Count;
                        }
                        b'0'..=b'9' => game_num = game_num * 10 + (c - b'0'),
                        _ => {
                            continue;
                        }
                    }
                }
                Seeking::Count => {
                    if c == &b' ' {
                        if count_val > 0 {
                            state = Seeking::Color;
                        }
                    } else {
                        count_val = count_val * 10 + (c - b'0');
                    }
                }
                Seeking::Color => match c {
                    b'r' => {
                        min_red = max(min_red, count_val as u32);
                        state = Seeking::Delimeter;
                    }
                    b'g' => {
                        min_green = max(min_green, count_val as u32);
                        state = Seeking::Delimeter;
                    }
                    b'b' => {
                        min_blue = max(min_blue, count_val as u32);
                        state = Seeking::Delimeter;
                    }
                    b' ' => {
                        continue;
                    }
                    b',' => {
                        unreachable!("comma while seeking a color");
                    }
                    b';' => {
                        unreachable!("semicolon while seeking a color");
                    }
                    _ => {
                        unreachable!("'{}' while seeking a color", *c as char);
                    }
                },
                Seeking::Delimeter => match c {
                    b',' => {
                        count_val = 0;
                        state = Seeking::Count;
                    }
                    b';' => {
                        count_val = 0;
                        state = Seeking::Count;
                    }
                    b'\n' => {
                        tally += min_red * min_green * min_blue;
                        count_val = 0;
                        game_num = 0;
                        min_red = 0;
                        min_green = 0;
                        min_blue = 0;
                        state = Seeking::Start;
                    }
                    _ => {
                        continue;
                    }
                },
                Seeking::Newline => {
                    if c == &b'\n' {
                        state = Seeking::Start;
                        count_val = 0;
                        game_num = 0;
                        min_red = 0;
                        min_green = 0;
                        min_blue = 0;
                    }
                }
            }
        }

        tally
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(8, 2286)]
    const input1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";
}
