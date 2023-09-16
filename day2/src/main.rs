use std::fs::read_to_string;

fn main() {
    let path = "src/input.txt";
    println!("Reading {}", path);
    let contents = read_lines(path);

    let mut matches: Vec<Match> = Vec::new();

    for value in contents.iter(){
        let readmatch : Vec<&str> = value.split(' ').collect::<Vec<&str>>();
        matches.push(Match{
            played: *readmatch.get(0).unwrap().chars().collect::<Vec<char>>().get(0).unwrap(),
            response: *readmatch.get(1).unwrap().chars().collect::<Vec<char>>().get(0).unwrap(),
        })
    }
    let mut totalscore = 0;

    for singlematch in matches{

        // A, X = rock
        // B, Y = paper
        // C, Z = scissors
        let mut score = 0;

        // this was for part one.
        /*score += match singlematch.response {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };*/

        score += match singlematch.played{
            'A' => match singlematch.response {
                'X' => 3,
                'Y' => 4,
                'Z' => 8,
                _ => 0,
            },
            'B' => match singlematch.response {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0
            },
            'C' => match singlematch.response {
                'X' => 2,
                'Y' => 6,
                'Z' => 7,
                _ => 0,
            },
            _ => 0,
        };
        totalscore += score;
    }
    println!("{:#?}", totalscore);
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Match {
    played: char,
    response: char
}