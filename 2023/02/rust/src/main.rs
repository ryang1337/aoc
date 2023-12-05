use std::collections::HashMap;

use std::cmp;
use text_io::read;
use text_io::scan;
// use text_io::try_read;

fn main() {
    let mut res = 0;
    loop {
        // let line: Result<String, _> = try_read!("{}\n");
        // let line = match line {
        //     Ok(v) => v,
        //     Err(_) => {
        //         break;
        //     }
        // };
        let line: String = read!("{}\n");
        if line == "" {
            break;
        }
        let mut line = line.bytes();
        let components: String;
        let game_num: i32;

        scan!(line => "Game {}: {}\n", game_num, components);
        let mut map = HashMap::new();
        map.insert("blue", 0);
        map.insert("green", 0);
        map.insert("red", 0);
        for component in components.split(";") {
            let components = component.trim();
            // let mut map = HashMap::new();
            // map.insert("blue", 14);
            // map.insert("green", 13);
            // map.insert("red", 12);
            for component in components.split(",") {
                let components = component.trim();
                let v = components.split_whitespace().collect::<Vec<_>>();
                map.insert(v[1], cmp::max(map[v[1]], v[0].parse::<i32>().unwrap()));
                // if map[v[1]] < 0 {
                //     valid = false;
                // }
            }
        }

        let power = map["red"] * map["blue"] * map["green"];
        res += power;

        // if valid {
        //     res += game_num;
        // }
    }

    println!("{res}");
}
