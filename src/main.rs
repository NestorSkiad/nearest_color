extern crate csv;
extern crate clap;

use std::collections::HashMap;
use std::error::Error;
use rayon::prelude::{IntoParallelIterator, ParallelSliceMut};
use rayon::iter::ParallelIterator;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "MultiThreadedGenerator")]
    command: String,
}


type Color = [i64; 3];

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

fn build_standard_color(name: String, color: Color) -> StandardColor {
    StandardColor {
        name,
        color,
    }
}

fn get_standard_colors() -> Result<Vec<StandardColor>, Box<dyn Error>> {
    let mut res: Vec<StandardColor> = Vec::with_capacity(865);
    let colors_str = include_str!("colors.csv");
    let mut rdr = csv::Reader::from_reader(colors_str.as_bytes());

    for record in rdr.records() {
        let record = record?;
        let color = [
            record[1].parse::<i64>()?,
            record[2].parse::<i64>()?,
            record[3].parse::<i64>()?
        ];
        res.push(build_standard_color(String::from(&record[0]), color));
    }
    Ok(res)
}

fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut res = map1.clone();
    for (k, v) in map2.into_iter() {
        match res.get(&k) {
            None => { res.insert(k, v); }
            Some(x) => { res.insert(k, x + v); }
        }
    }
    res
}

fn nearest_color_single(standard_colors: &Vec<StandardColor>, color: Color) -> Result<StandardColor, Box<dyn Error>> {
    Ok(standard_colors.into_iter()
        .map(|std| (std.clone(), euclidean_distance(&std.color, &color)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).expect("Failed ordering in nearest_color_single")).expect("nearest_color_single failed to find minimum")
        .0)
}

fn run_single_threaded(standard_colors: Vec<StandardColor>, color_num: &i32) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    println!("Running SingleThreaded");
    let mut distribution = HashMap::with_capacity(865);

    for color in (0..*color_num).map(|x| num_to_color(x)) {
        let nearest_color = nearest_color_single(&standard_colors, color)?.name;

        match distribution.get(&nearest_color) {
            Some(n) => distribution.insert(nearest_color, n + 1),
            None => distribution.insert(nearest_color, 1)
        };
    }

    Ok(distribution)
}

fn run_multithreaded_generator(standard_colors: Vec<StandardColor>, color_num: &i32) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    println!("Running MultiThreadedGenerator");

    let res: Vec<String> = (0..*color_num).into_par_iter()
        .map(|x| num_to_color(x))
        .map(|x| nearest_color_single(&standard_colors, x).unwrap().name)
        .collect();

    let mut distribution = HashMap::with_capacity(865);

    for color in res.into_iter() {
        match distribution.get(&color) {
            Some(n) => distribution.insert(color, n + 1),
            None => distribution.insert(color, 1)
        };
    }

    Ok(distribution)
}

fn run_multithreaded_merge(standard_colors: Vec<StandardColor>, color_num: &i32) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    println!("Running MultiThreadedMerge");

    Ok((0..*color_num).into_par_iter()
        .map(|x| num_to_color(x))
        .map(|x| nearest_color_single(&standard_colors, x).unwrap().name)
        .map(|x| {
            HashMap::from([(x, 1)])
        })
        .reduce(|| HashMap::new(), |x, y| merge_maps(x, y)))
}


fn main() -> Result<(), Box<dyn Error>> {
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

    let standard_colors: Vec<StandardColor> = get_standard_colors()?;

    println!("Named colors: {:?}", standard_colors);

    let distribution = match cli.command.to_lowercase().as_str() {
        "singlethreaded" => run_single_threaded(standard_colors, &colors)?,
        "percolor" => HashMap::new(),
        "multithreadedgenerator" => run_multithreaded_generator(standard_colors, &colors)?,
        "multithreadedmerge" | _ => run_multithreaded_merge(standard_colors, &colors)?
    };

    let mut dist_vec: Vec<(String, i32)> = distribution.into_iter().collect();

    dist_vec.par_sort_unstable_by(|x, y| y.1.cmp(&x.1));

    println!("{:?}", dist_vec);

    let total_colors: i32 = dist_vec.into_iter().map(|x| x.1).sum();
    assert_eq!(colors, total_colors);
    //println!("{:?}", nearest_color_single(&standard_colors, [192, 58, 88]))

    Ok(())
}
