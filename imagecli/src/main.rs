extern crate imagelib;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: imagecli {{blur|invert|greyscale|hflip|rotate|vflip}} <in-image.png> <out-image.png>");
        process::exit(1);
    }

    let operation = &args[1];
    let fname = &args[2];
    let ofname = &args[3];
    let file_bytes = fs::read(fname).unwrap();

    let xformed = match operation.as_str() {
        "blur" => imagelib::do_transform_img(&file_bytes, imagelib::Operation::Blur),
        "invert" => imagelib::do_transform_img(&file_bytes, imagelib::Operation::Invert),
        "greyscale" => imagelib::do_transform_img(&file_bytes, imagelib::Operation::Greyscale),
        "hflip" => imagelib::do_transform_img(&file_bytes, imagelib::Operation::HFlip),
        "rotate" => imagelib::do_transform_img(&file_bytes, imagelib::Operation::Rotate),
        "vflip" => imagelib::do_transform_img(&file_bytes, imagelib::Operation::VFlip),
        _ => { println!("{}: Unknown operation: {}", &args[0], operation); process::exit(1); }
    };
    
    fs::write(ofname, xformed).unwrap();
}
