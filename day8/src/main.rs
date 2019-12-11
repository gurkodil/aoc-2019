// Day 8
// https://adventofcode.com/2019/day/8
mod parser;

fn main() {
    let input = parser::parse_file("data/08.dat".to_string());

    const W: usize = 25;
    const H: usize = 6;

    let layers: Vec<Vec<u32>> = input.chunks(W * H).clone().map(|x| x.to_vec()).collect();

    // Findning the layer with the least zeroes
    let record_low = layers
        .iter()
        .map(|layer| layer.iter().filter(|n| **n == 0).count())
        .enumerate()
        .min_by(|(_, n1), (_, n2)| n1.cmp(n2));

    // Part one
    if let Some((index, _)) = record_low {
        let nr_of_twos = layers[index].iter().filter(|&i| *i == 2).count();
        let nr_of_ones = layers[index].iter().filter(|&i| *i == 1).count();
        println!("Part one answer: {}", nr_of_twos * nr_of_ones);
    }

    // Part two
    let mut decoded_image = layers[0].clone();
    for layer in layers.iter() {
        for (i, pixel) in layer.iter().enumerate() {
            if decoded_image[i] == 2 {
                decoded_image[i] = *pixel;
            }
        }
    }

    // Print result part two
    for (index, pixel) in decoded_image.iter().enumerate() {
        if index % W == 0 {
            println!("\n");
        }

        let color = if *pixel == 1 { "|" } else { " " };

        print!(" {} ", color);
    }

    println!("\n\nPart two answer^");
}
