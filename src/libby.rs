// contains code to
// 1 - parse command line args
// 2 - check that they are correct number of params
// 3 - return appropriate error message if params are of invalid type 
// 4 - usage message if no params are provided
pub mod libby {

    pub struct Config {
        pub task_name: String,    
        pub os_name: String,
        pub image_name: String,
    }

    pub fn parse_args(args: &[String]) -> Config {
        let task_name = args[1].clone();
        let os_name = args[2].clone();
        let image_name = args[3].clone();

        Config {task_name, os_name, image_name }
    }
}