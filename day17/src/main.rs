use std::ops::RangeInclusive;

fn main() {
    let horizontal = 195..=238;
    let vertical = -93..=-67;

    let mut max_height = None;

    for y_velocity in -100..1000 {
        for x_velocity in 1..1000 {
            let result = shoot(x_velocity, y_velocity, &horizontal, &vertical);

            if result.is_none() {
                continue;
            }

            max_height = result
                .map(|max_y|
                    match max_height {
                        Some(previous) => max_y.max(previous),
                        None => max_y,
                    }
                );
        }
    }

    println!("part1: max_height={0:?}", max_height);
}

fn shoot(x_velocity: i32, y_velocity: i32, horizontal: &RangeInclusive<i32>, vertical: &RangeInclusive<i32>) -> Option<i32> {
    let mut x_velocity = x_velocity;
    let mut y_velocity = y_velocity;

    let mut max_y = 0;

    let mut x = 0;
    let mut y = 0;

    loop {
        x += x_velocity;
        y += y_velocity;

        if y > max_y {
            max_y = y;
        }

        if x_velocity > 0 {
            x_velocity -= 1;
        }

        y_velocity -= 1;

        if horizontal.contains(&x) && vertical.contains(&y) {
            return Some(max_y);
        }

        if x > *horizontal.end() || y < *vertical.start() {
            return None;
        }
    }
}
