extern crate csv;
#[macro_use]
extern crate serde;

use std::collections::HashMap;


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

fn to_standard_color(dsc: DeserialisedStandardColor) -> StandardColor {
    StandardColor { name: String::from(&dsc.name), color: [dsc.r, dsc.g, dsc.b] }
}

fn get_standard_colors() -> Vec<StandardColor> {
    let mut res: Vec<StandardColor> = Vec::with_capacity(865);
    let mut rdr = csv::Reader::from_path("C:/Users/Nestor/IdeaProjects/nearest_color/colors.csv");

    for result in rdr.unwrap().deserialize() {
        let std_color: DeserialisedStandardColor = result.unwrap();
        res.push(to_standard_color(std_color))
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


fn main() {
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

    /*let sample_standard = StandardColor { name: String::from("Sample"), color: [255, 255, 255] };
    let other_standard = StandardColor { name: String::from("Other"), color: [0, 0, 0] };*/
    //let mut output = String::new();
    let mut distribution = HashMap::with_capacity(865);

    for color in color_generator().into_iter() {
        //output.push_str(&format!("nearest color to {:?} is {:?}\n", color, nearest_color_single(&standard_colors, color).name));

        let nearest_color = nearest_color_single(&standard_colors, color).name;

        match distribution.get(&nearest_color) {
            Some(n) => distribution.insert(nearest_color, n + 1),
            None => distribution.insert(nearest_color, 1)
        };
    }

    let mut dist_vec: Vec<(String, i32)> = distribution.into_iter().collect();

    dist_vec.sort_by_key(|x| x.1);
    dist_vec.reverse();

    //println!("{}", output);
    println!("{:?}", dist_vec);
    //println!("{:?}", nearest_color_single(&standard_colors, [192, 58, 88]))
}
