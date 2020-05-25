
// std libraries

use std::fs::File;
use std::io::{BufRead, BufReader};

// external crates
#[macro_use] extern crate bmp;
use bmp::{Image, Pixel};


fn main() {
        // open file
    // NOTE: unwrap either returns the expected element or a "panic"
    let mut f  = BufReader::new(File::open("bbost9_hard.txt").unwrap()); 
    
    // read txt in as string
    let mut raw_map = String::new();
    f.read_line(&mut raw_map).unwrap();
    
    // convert string into f32 vector
    let raw_map = raw_map.replace(&['[',']',' '][..],"");
    let map = raw_map.split(",").filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>();
    
    // declare min and max variables to store min and max voltages
    let mut min = map[0];
    let mut max = map[0];
    let mut ratio = 0.0;
    
    // iterate through the vector to get min and max values
    for i in map.clone(){
        if i < min{
            min = i;
        }
        else if i > max {
            max = i;
        }
        ratio = min/max;
        // print!("{} ", i);
    }
    // scale max value to int reflecting rgb value
    let scale = (255.0 / max) as i64;
    let scale_f = scale as f32;
    let mut scaled_map = Vec::new();
    
    // create a new map with rgb compatible integers
    // we'll refer to this scaled map as the grayscale
    for i in map.clone() {
        let temp = (i * scale_f) as i64;
        scaled_map.push(temp);
        //print!("{} ", temp);
    }
    println!("\n{} / {} = {}", min, max, ratio);

    // determine the resolution of the image by retrieving the 
    // square root of total elements
    let res = (map.len() as f32).sqrt() as u32;

    // initialize bitmap image
    let mut img = Image::new(res,res);

    let mut counter = 0;
    // let mut row
    // use ndarray::Array2;
    // let mut multi_map = Array2::<i64>::zeros((res,res));

    // enumerate through bitmap while sequentially setting 
    // each pixel to the integer value in the grayscale array
    for (x,y) in img.coordinates(){
        img.set_pixel(x,y, px!(scaled_map[counter], scaled_map[counter],scaled_map[counter]));
        counter += 1;

    }

    // save the image to a file
    let _ = img.save("img.bmp");
}
