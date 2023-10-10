use std::{ error::Error, fs};

pub struct Config {
  pub file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 2 {
          Err("Not Enough arguments!")
      } else {
          Ok(
              Config {
                  file_path: args[1].clone()
              }
          )
      }
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  let contents = fs::read_to_string(config.file_path)?;
  
  println!("Read Contents: \n{contents}");

  Ok(())
}