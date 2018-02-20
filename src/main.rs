extern crate image;

// extern crate libby;
use std::env;
use std::fs::File;
use image::{FilterType, GenericImage, Pixel};

pub mod libby;

fn main() {
    //ensure programs has atleast some params ..else show mesg and exit

    //check images passed in have the right sizes

    // show progress on command line while images are being created?

    // aditional functionality - create thumbnails
    // create rounded profile images

    // let img = image::open("test.png").expect("Opening image failed");
    // let filtered = img.fliph();
    // let mut out = File::create("out.png").unwrap();
    // filtered.save(&mut out, image::PNG).expect("Saving image failed");

    // let img2 = image::open("test.png").expect("Opening image failed");
    // let thumbnail = img2.resize(200,200, FilterType::Lanczos3);
    // let mut thumper = File::create("resized.png").unwrap();
    // thumbnail.save(&mut thumper, image::PNG).expect("Saving image failed");
    //resizer splash android /path/to/img
    //command task os path

    let args: Vec<String> = env::args().collect();
    let arg_struct = libby::libby::parse_args(&args);

     println!("Argument 1 is: {}", arg_struct.task_name);
     println!("Argument 2 is: {}", arg_struct.os_name);
     println!("Argument 3 is: {}", arg_struct.image_name);
}
