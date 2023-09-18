extern crate csv;
extern crate serde;
extern crate clap;

use std::collections::HashMap;
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "MultiThreadedGenerator")]
    command: String,
}



type Color = [i64; 3];

fn color_to_str(c: Color) -> String {
    format! {"R{} G{} B{}", c[0], c[1], c[2]}
}

fn euclidean_distance(vec_a: &Color, vec_b: &Color) -> f64 {
    let mut sum: i64 = 0;

    for (a, b) in vec_a.into_iter().zip(vec_b.into_iter()) {
        sum += (a - b).pow(2)
    }

    (sum as f64).sqrt()
}

struct ColorGenerator {
    curr: Color,
}

impl Iterator for ColorGenerator {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        if self.curr[2] < 255 {
            self.curr[2] += 1;
        } else {
            self.curr[2] = 0;

            if self.curr[1] < 255 {
                self.curr[1] += 1;
            } else {
                self.curr[1] = 0;
                self.curr[0] += 1;
            }
        }

        if current[0] <= 255 {
            Some(current)
        } else {
            None
        }
    }
}

fn color_generator() -> ColorGenerator {
    ColorGenerator { curr: [0; 3] }
}

// A Lot faster than the generator
fn num_to_color(num: i32) -> Color {
    let mut c: Color = [0, 0, 0];

    c[2] = (num % 256) as i64;
    let mut div: i64 = (num / 256) as i64;
    c[1] = div % 256;
    div = div / 256;
    c[0] = div % 256;

    c
}

#[derive(Debug, Clone)]
struct StandardColor {
    name: String,
    color: Color,
}

#[derive(serde::Deserialize, Debug)]
struct DeserialisedStandardColor {
    name: String,
    r: i64,
    g: i64,
    b: i64,
}

impl DeserialisedStandardColor {
    fn to_standard_color(self) -> StandardColor {
        StandardColor { name: String::from(self.name), color: [self.r, self.b, self.g]}
    }
}

fn get_standard_colors() -> Vec<StandardColor> {
    let mut res: Vec<StandardColor> = Vec::with_capacity(865);
    let rdr = csv::Reader::from_path("C:/Users/Nestor/IdeaProjects/nearest_color/colors.csv"); // todo: make relative and use include like in nearest color GUI

    for result in rdr.unwrap().deserialize() {
        let std_color: DeserialisedStandardColor = result.unwrap();
        res.push(std_color.to_standard_color())
    }
    res
}

fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut res = map1.clone();
    for (k, v) in map2.into_iter() {
        match res.get(&k) {
            None => { res.insert(k, 1); }
            Some(x) => { res.insert(k, x + v); }
        }
    }
    res
}

fn nearest_color_single(standard_colors: &Vec<StandardColor>, color: Color) -> StandardColor {
    standard_colors.into_iter()
        .map(|std| (std.clone(), euclidean_distance(&std.color, &color)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .0
}

fn run_single_threaded(standard_colors: Vec<StandardColor>, color_num: &i32) -> HashMap<String, i32> {
    println!("Running SingleThreaded");
    let mut distribution = HashMap::with_capacity(865);

    for color in (0..*color_num).map(|x| num_to_color(x)) {
        let nearest_color = nearest_color_single(&standard_colors, color).name;

        match distribution.get(&nearest_color) {
            Some(n) => distribution.insert(nearest_color, n + 1),
            None => distribution.insert(nearest_color, 1)
        };
    }

    distribution
}

fn run_multithreaded_generator(standard_colors: Vec<StandardColor>, color_num: &i32) -> HashMap<String, i32> {
    println!("Running MultiThreadedGenerator");

    let mut distribution = HashMap::with_capacity(865);

    let res: Vec<String> = (0..*color_num).into_par_iter()
        .map(|x| num_to_color(x))
        .map(|x| nearest_color_single(&standard_colors, x).name)
        .collect();

    for color in res.into_iter() {
        match distribution.get(&color) {
            Some(n) => distribution.insert(color, n + 1),
            None => distribution.insert(color, 1)
        };
    }

    distribution
}


fn main() {
    let cli = Cli::parse();
    let colors: i32 = 2_i32.pow(8).pow(3);
    println!("number of 8 bit colors is {colors}");

    /*let vector_one: Color = [154, 241, 4];
    let vector_two: Color = [168, 2, 257];

    let res = euclidean_distance(&vector_one, &vector_two);

    let mut iter = color_generator().into_iter();
    let first = iter.next().unwrap();
    let last = iter.last().unwrap();

    println!("euclidean distance is {res}");
    println!("first color is {} and last color is {}", color_to_str(&first), color_to_str(&last));
    println!("they have a euclidian distance of {}", euclidean_distance(&first, &last));

    let _all_colors: Vec<Color> = color_generator().into_iter().collect(); */

    let standard_colors = get_standard_colors();

    println!("Named colors: {:?}", standard_colors);

    let distribution = match cli.command.as_str() {
        "SingleThreaded" => { run_single_threaded(standard_colors, &colors) }
        "PerColor" => { HashMap::new() }
        "MultiThreadedGenerator" | _ => { run_multithreaded_generator(standard_colors, &colors) }
    };

    /*let distribution = (0..colors).into_par_iter()
        .map(|x| num_to_color(x))
        .map(|x| nearest_color_single(&standard_colors, x).name)
        .map(|x| {
            HashMap::from([(x, 1)])
        })
        .reduce(|| HashMap::new(), |x, y| merge_maps(x, y));*/

    let mut dist_vec: Vec<(String, i32)> = distribution.into_iter().collect();

    dist_vec.sort_by_key(|x| x.1);
    dist_vec.reverse();

    //println!("{}", output);
    println!("{:?}", dist_vec);

    let total_colors: i32 = dist_vec.into_iter().map(|x| x.1).sum();
    assert_eq!(colors, total_colors);
    //println!("{:?}", nearest_color_single(&standard_colors, [192, 58, 88]))
}
