extern crate image;

// extern crate libby;
use std::env;
use std::process;

pub mod libby;
pub mod resize;

fn main() {
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

    let args: Vec<String> = env::args().collect();
    
    let config = libby::libby::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if config.task_name == "splashes" {
        //do splashes
        create_splashes(config)
    } else {
        //do icons
        create_icons(config)
    }
}

fn create_splashes(config: libby::libby::Config) {
    println!("Argument 1 is: {}", config.task_name);
     println!("Argument 2 is: {}", config.os_name);
//     println!("Argument 3 is: {}", config.image_name);
}

fn create_icons(config: libby::libby::Config) {
    println!("Argument 1 is: {}", config.task_name);
     println!("Argument 2 is: {}", config.os_name);
     println!("Argument 3 is: {}", config.image_name);
    //if os is IOS
    if config.os_name == "ios" {
        //validate image ( size, extention - only jpeg, png)
    } else {
        //android
    }    

    //if os is android
}