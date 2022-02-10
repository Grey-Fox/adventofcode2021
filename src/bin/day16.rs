use adventofcode2021::get_lines;
use hex;
use std::cmp::min;
use std::error::Error;

#[derive(Debug)]
struct Package {
    version: usize,
    package_type: usize,
    packages: Vec<Package>,
    value: Option<usize>,
}

impl Package {
    fn calculate(&self) -> usize {
        match self.package_type {
            0 => {
                let mut s = 0;
                for p in &self.packages {
                    s += p.calculate();
                }
                s
            }
            1 => {
                let mut s = 1;
                for p in &self.packages {
                    s *= p.calculate();
                }
                s
            }
            2 => self.packages.iter().map(|p| p.calculate()).min().unwrap(),
            3 => self.packages.iter().map(|p| p.calculate()).max().unwrap(),
            4 => self.value.unwrap(),
            5 => (self.packages[0].calculate() > self.packages[1].calculate()) as usize,
            6 => (self.packages[0].calculate() < self.packages[1].calculate()) as usize,
            7 => (self.packages[0].calculate() == self.packages[1].calculate()) as usize,
            _ => 999,
        }
    }
}

#[derive(Debug)]
struct Transmission {
    bytes: Vec<u8>,
}

impl Transmission {
    fn new(s: &str) -> Transmission {
        Transmission {
            bytes: hex::decode(s).unwrap(),
        }
    }
    fn read_bits(&self, pos: &mut usize, len: usize) -> usize {
        if len <= 0 {
            return 0;
        }
        let b = self.bytes[*pos / 8];
        let p = *pos % 8;
        let to_read = min(len, 8 - p);
        let res = ((b << p >> p >> (8 - p - to_read)) as usize) << (len - to_read);
        *pos += to_read;
        res + self.read_bits(pos, len - to_read)
    }
    fn parse_literal_value(&self, pos: &mut usize) -> usize {
        let mut value = 0;
        loop {
            value = value << 4;
            let part = self.read_bits(pos, 5);
            value += part & 15;
            if (part & 16) == 0 {
                break value;
            }
        }
    }
    fn parse_sub_packages(&self, pos: &mut usize) -> Vec<Package> {
        let length_type_id = self.read_bits(pos, 1);
        let mut res = Vec::new();
        if length_type_id == 0 {
            let length_in_bits = self.read_bits(pos, 15);
            let cur = *pos;
            while *pos < cur + length_in_bits {
                res.push(self.parse_package(pos));
            }
        } else {
            let number = self.read_bits(pos, 11);
            for _ in 0..number {
                res.push(self.parse_package(pos));
            }
        }
        res
    }

    fn parse_package(&self, pos: &mut usize) -> Package {
        let version = self.read_bits(pos, 3);
        let package_type = self.read_bits(pos, 3);
        let mut packages = Vec::new();
        let mut value = None;

        if package_type == 4 {
            value = Some(self.parse_literal_value(pos));
        } else {
            packages = self.parse_sub_packages(pos);
        }

        Package {
            version,
            package_type,
            packages,
            value,
        }
    }
}

fn sum_versions(package: &Package) -> usize {
    let mut res = package.version;
    for sub_package in &package.packages {
        res += sum_versions(&sub_package);
    }
    res
}

fn task1(package: &Package) {
    println!("Task1: {}", sum_versions(&package));
}

fn task2(package: &Package) {
    println!("Task2: {}", package.calculate());
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_lines()?;
    for line in lines {
        if line.len() > 77 {
            println!("{}...", &line[..77]);
        } else {
            println!("{}", line);
        }
        let t = Transmission::new(&line);
        let p = t.parse_package(&mut 0);
        task1(&p);
        task2(&p);
    }
    Ok(())
}
