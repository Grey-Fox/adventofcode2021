use adventofcode2021::get_lines;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let commands: Vec<String> = get_lines()?.collect();
    task1(&commands);
    task2(&commands);
    Ok(())
}

fn task1(report: &Vec<String>) {
    let mut one_count = vec![0; report[0].len()];
    for line in report {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                one_count[i] += 1
            }
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for c in one_count {
        if c > (report.len() / 2) {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    let gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Task1: {}", gamma_rate * epsilon_rate);
}

fn rec<'a, T>(
    report_part: &Vec<&'a String>,
    i: usize,
    condition: fn(usize, usize, usize) -> T,
) -> &'a String
where
    T: FnMut(&&String) -> bool,
{
    if report_part.len() == 0 {
        panic!("bad condition")
    }
    if report_part.len() == 1 {
        return &report_part[0];
    }
    let mut c: usize = 0;
    for line in report_part {
        if line.chars().nth(i).unwrap() == '1' {
            c += 1;
        }
    }
    let n = report_part.len();
    let mut filter_func = condition(c, n, i);
    let mut filtered = Vec::<&String>::new();
    for line in report_part {
        if filter_func(&line) {
            filtered.push(line);
        }
    }
    return rec(&filtered, i + 1, condition);
}

fn find_f(i: usize, c: char) -> impl FnMut(&&String) -> bool {
    move |&x| x.chars().nth(i).unwrap() == c
}

fn task2(report: &Vec<String>) {
    let oxygen = rec(&report.iter().collect(), 0, |c, n, i| {
        if c >= n - c {
            find_f(i, '1')
        } else {
            find_f(i, '0')
        }
    });
    let co2 = rec(&report.iter().collect(), 0, |c, n, i| {
        if c >= n - c {
            find_f(i, '0')
        } else {
            find_f(i, '1')
        }
    });
    let oxygen = isize::from_str_radix(oxygen, 2).unwrap();
    let co2 = isize::from_str_radix(co2, 2).unwrap();
    println!("Task2: {}", oxygen * co2);
}
