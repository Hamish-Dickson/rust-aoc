use std::collections::HashMap;
use std::fs::read_to_string;
use std::str;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut prioritypairs : HashMap<char, i32> = HashMap::new();
    for i in 0..=51{
        prioritypairs.insert(*alphabet.chars().collect::<Vec<char>>().get(i).unwrap(), (i + 1) as i32);
    }

    let rucksacks = read_lines("src/input.txt");

    let mut commonRucksacks: Vec<Rucksack> = Vec::new();
    for bag in rucksacks{
        let compartments = bag.split_at(bag.chars().count()/2);
        let mut compartmentOne = compartments.0;
        let mut compartmentTwo = compartments.1;

        let mut commonBagItems: String = String::from("");

        let compartmentOneChars = compartmentOne.chars().collect::<Vec<char>>();
        for i in 0..compartmentOneChars.len(){
            let item = compartmentOneChars.get(i).unwrap();
            if (!commonBagItems.contains(*item)) && (compartmentTwo.contains(*item)) {
                commonBagItems.push(*item);
            }
        }

        commonRucksacks.push(Rucksack{
            id: bag,
            commonItems: commonBagItems,
        });
    }

    let mut totalPriority = 0;

    for rucksack in commonRucksacks{
        let sharedItems : Vec<char> = rucksack.commonItems.chars().collect();
        for item in sharedItems{
            let priority = prioritypairs.get(&item);
            if !priority.is_none() {
                totalPriority += priority.unwrap();
            }
        }
    }
    println!("{:#?}", totalPriority);
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[derive(Debug)]
struct Rucksack {
    id: String,
    commonItems: String,
}
