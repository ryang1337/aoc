use std::{
    cmp,
    collections::HashMap,
    io::{self, BufRead, Lines},
};
use text_io::scan;

fn populate_map<B: BufRead>(map: &mut HashMap<(i64, i64), i64>, it: &mut Lines<B>) {
    loop {
        let line = it.next();
        let line = match line {
            Some(l) => l,
            None => break,
        };
        let line = line.unwrap();
        if line == "" {
            break;
        }

        let data: Vec<i64> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let range: (i64, i64) = (data[1], (data[1] + data[2] - 1));
        map.insert(range, data[0] - data[1]);
    }
}

fn get_mapping(map: &HashMap<(i64, i64), i64>, n: i64) -> i64 {
    for (&range, &offset) in map {
        if n >= range.0 && n <= range.1 {
            return n + offset;
        }
    }

    return n;
}

pub fn solve() {
    let mut it = io::stdin().lines();

    let seeds: Vec<i64>;
    let mut seed_soil: HashMap<(i64, i64), i64> = HashMap::new();
    let mut soil_fert: HashMap<(i64, i64), i64> = HashMap::new();
    let mut fert_water: HashMap<(i64, i64), i64> = HashMap::new();
    let mut water_light: HashMap<(i64, i64), i64> = HashMap::new();
    let mut light_temp: HashMap<(i64, i64), i64> = HashMap::new();
    let mut temp_hum: HashMap<(i64, i64), i64> = HashMap::new();
    let mut hum_loc: HashMap<(i64, i64), i64> = HashMap::new();

    let line = it.next().unwrap().unwrap();
    let data: String;

    scan!(line.bytes() => "seeds: {}\n", data);
    seeds = data
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    it.next();
    it.next();
    populate_map(&mut seed_soil, &mut it);
    it.next();
    populate_map(&mut soil_fert, &mut it);
    it.next();
    populate_map(&mut fert_water, &mut it);
    it.next();
    populate_map(&mut water_light, &mut it);
    it.next();
    populate_map(&mut light_temp, &mut it);
    it.next();
    populate_map(&mut temp_hum, &mut it);
    it.next();
    populate_map(&mut hum_loc, &mut it);

    let mut lowest_location = i64::MAX;

    for seed in seeds {
        let soil = get_mapping(&seed_soil, seed);
        let fert = get_mapping(&soil_fert, soil);
        let water = get_mapping(&fert_water, fert);
        let light = get_mapping(&water_light, water);
        let temp = get_mapping(&light_temp, light);
        let hum = get_mapping(&temp_hum, temp);
        let loc = get_mapping(&hum_loc, hum);

        lowest_location = cmp::min(lowest_location, loc);
    }

    println!("{lowest_location}");
}
