use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let fish: Vec<i32> = lines.iter().next()
        .expect("Could not read fish")
        .split(',')
        .map(|token| token.parse::<i32>().unwrap())
        .collect();

    {
        let mut fish = fish.clone();

        println!("fish={0:?}", fish);

        for _day in 0..80 {
            let len = fish.len();

            for index in 0..len {
                let current_fish = fish[index];

                match current_fish {
                    0 => {
                        fish[index] = 6;
                        fish.push(8);
                    }
                    _ => {
                        fish[index] -= 1;
                    }
                }
            }
        }

        let number_of_fish = fish.len();
        println!("part1: number of fish={0:?}", number_of_fish);
        assert_eq!(number_of_fish, 380243);
    }

    {
        let mut fish_age = fish.clone().into_iter()
            .fold(vec![0; 9], |mut ages, value| {
                ages[value as usize] += 1;
                ages
            });

        println!("fish_age={0:?}", fish_age);

        for _day in 0..256 {
            let spawning_fish = fish_age[0];

            // Fish with age 0 will spawn a new fish with age 8 today
            fish_age.rotate_left(1);

            // They are also respawned with age 6
            fish_age[6] += spawning_fish;
        }

        let number_of_fish: usize = fish_age.iter().sum();
        println!("part2: number of fish={0:?}", number_of_fish);
        assert_eq!(number_of_fish, 1708791884591);
    }
}
