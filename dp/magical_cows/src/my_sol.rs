#![allow(dead_code)]

use std::io::stdin;

fn read_cow_input(cows_per_farm: &mut Vec<usize>, visit_days: &mut Vec<usize>) -> usize {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("could not read input first line");

    let vec_input = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (max_cow_per_farm, n_farms_0, m_days) = (vec_input[0], vec_input[1], vec_input[2]);

    for _ in 0..n_farms_0 {
        input.clear();
        stdin()
            .read_line(&mut input)
            .expect("could not read cow quantity on a farm");

        cows_per_farm.push(input.trim().parse::<usize>().unwrap());
    }

    for _ in 0..m_days {
        input.clear();
        stdin()
            .read_line(&mut input)
            .expect("could not read days of visit");

        visit_days.push(input.trim().parse::<usize>().unwrap());
    }

    return max_cow_per_farm;
}

fn magical_cow_day(cows_per_farm: &Vec<usize>, day: usize, max_cow_per_farm: usize) -> usize {
    let mut n_farms = cows_per_farm.len();

    for cows in cows_per_farm {
        let multiplier = max_cow_per_farm / *cows;
        let power: f32 = day as f32 - (multiplier as f32).log2();
        if power < 0.0 {
            continue;
        }
        n_farms += 2_f32.powf(power).ceil() as usize - 1;
    }

    return n_farms;
}

fn magical_cow(
    cows_per_farm: &Vec<usize>,
    visit_days: &Vec<usize>,
    max_cow_per_farm: usize,
) -> Vec<usize> {
    let mut farms_per_day: Vec<usize> = vec![0; visit_days.len()];
    for (i, day) in visit_days.iter().enumerate() {
        farms_per_day[i] = magical_cow_day(cows_per_farm, *day, max_cow_per_farm);
    }

    return farms_per_day;
}

fn test() {
    let mut cows_per_farm: Vec<usize> = Vec::new();
    let mut visit_days: Vec<usize> = Vec::new();
    let max_cow_per_farm: usize = read_cow_input(&mut cows_per_farm, &mut visit_days);
    //let max_cow_per_farm: usize = 3;
    //let cows_per_farm: Vec<usize> = vec![];
    //let visit_days: Vec<usize> = vec![0, 1, 2];
    let farms_per_day = magical_cow(&cows_per_farm, &visit_days, max_cow_per_farm);
    for farm in farms_per_day {
        println!("{farm}");
    }
}
