#![allow(dead_code)]

mod my_sol;
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

fn dp_magical_cows_construct(
    cows_per_farm: &[usize],
    max_cpf: usize,
    max_day: usize,
) -> Vec<Vec<usize>> {
    let mut table_day_vs_farms_with_n_cows: Vec<Vec<usize>> = vec![vec![0; max_cpf]; max_day + 1];

    for farm in 0..cows_per_farm.len() {
        table_day_vs_farms_with_n_cows[0][cows_per_farm[farm] - 1] += 1;
    }

    for row_day in 1..=max_day {
        for col_farm_with_n in 0..max_cpf {
            if (col_farm_with_n + 1) <= max_cpf / 2 {
                table_day_vs_farms_with_n_cows[row_day][((col_farm_with_n + 1) * 2) - 1] +=
                    table_day_vs_farms_with_n_cows[row_day - 1][col_farm_with_n];
            } else {
                table_day_vs_farms_with_n_cows[row_day][col_farm_with_n] +=
                    2 * table_day_vs_farms_with_n_cows[row_day - 1][col_farm_with_n];
            }
        }
    }

    return table_day_vs_farms_with_n_cows;
}

fn query_day(table: &[Vec<usize>], day: usize) -> usize {
    let mut sum_day: usize = 0;
    for col in 0..table[0].len() {
        sum_day += table[day][col];
    }

    return sum_day;
}

fn main() {
    let mut cows_per_farm: Vec<usize> = Vec::new();
    let mut visit_days: Vec<usize> = Vec::new();
    let max_cow_per_farm: usize = read_cow_input(&mut cows_per_farm, &mut visit_days);
    let last_day = visit_days[visit_days.len() - 1];
    let table = dp_magical_cows_construct(&cows_per_farm, max_cow_per_farm, last_day);
    for day in visit_days {
        let n_farms = query_day(&table, day);
        println!("{n_farms}");
    }
}
