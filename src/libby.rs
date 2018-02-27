// 2 - check that they are correct number of params
// 3 - return appropriate error message if params are of invalid type 

pub mod libby {
    pub struct Config {
        pub task_name: String,    
        pub os_name: String,
        pub image_name: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            // check if enough params are passed
            if args.len() < 3 {                
                return Err("not enough arguments")
            }

            let task_name = args[1].clone();
            let os_name = args[2].clone();

            //if task is splashes..then there wont be a 3rd arg
            let image_name = args[3].clone();

            //in this case need to set a default blank value for image_name

            Ok(Config {task_name, os_name, image_name })
        }
    }
}