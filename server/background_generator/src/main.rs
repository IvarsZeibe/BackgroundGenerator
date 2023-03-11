use std::env;

use background_generator::{chains_generator, triangles_generator};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Must have one argument, had {}", args.len());
    }
    let choosen_mode = &args[1];

    let image = if choosen_mode == "0" {
        triangles_generator::generate(
            500,
            500,
            15,
            [255, 0, 0],
            [0, 255, 0],
            1,
            triangles_generator::TriangleGeneratorMode::Diagonal,
        )
    } else if choosen_mode == "1" {
        chains_generator::generate(
            1920,
            1080,
            1000,
            5,
            5.,
            [0, 0, 0],
            [0, 0, 0],
            [255, 0, 0],
            1,
        )
    } else {
        panic!("No generator found");
    };
    image.save("./image.png").expect("Saving failed");
    println!("Success");
}
