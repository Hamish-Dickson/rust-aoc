use std::fs::read_to_string;

fn main() {
    let path = "src/input.txt";
    println!("Reading {}", path);
    let contents = read_lines(path);

    let mut elves: Vec<Elves> = Vec::new();

    let mut i = 0;
    while i < 999999 {
        let mut elf: Elves = Elves {
            id: i,
            calories: 0,
        };
        for value in contents.iter(){
            if !value.is_empty() {
                elf.calories += str::parse::<i32>(value).unwrap().abs();
            }else{
                i = i + 1;
                elves.push(elf);
                elf = Elves{
                    id: i,
                    calories: 0,
                };
            }
        }
        break;
    }
    let mut first:i32 = 0;
    let mut second :i32 = 0;
    let mut third :i32 = 0;
    for elf in elves.iter(){
        if elf.calories > first {
            third = second;
            second = first;
            first = elf.calories;
        }
        else if elf.calories > second {
            third = second;
            second = elf.calories
        }
        else if elf.calories > third {
            third = elf.calories
        }
    }
    //todo: the above can surely be done better, but i know nothing about rust.

    let total = first + second + third;
    println!("{}", total);


}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[derive(Clone, Copy)]
struct Elves {
    id: i32,
    calories: i32
}
