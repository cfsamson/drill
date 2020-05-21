use yaml_rust::{Yaml, YamlLoader};

use crate::reader;

const NITERATIONS: i64 = 1;
const NRAMPUP: i64 = 0;

pub struct Config {
  pub base: String,
  pub concurrency: i64,
  pub iterations: i64,
  pub relaxed_interpolations: bool,
  pub no_check_certificate: bool,
  pub rampup: i64,
  pub quiet: bool,
  pub nanosec: bool,
}

impl Config {
  pub fn new(path: &str, relaxed_interpolations: bool, no_check_certificate: bool, quiet: bool, nanosec: bool) -> Config {
    let config_file = reader::read_file(path);

    let config_docs = YamlLoader::load_from_str(config_file.as_str()).unwrap();
    let config_doc = &config_docs[0];

    let iterations = read_i64_configuration(config_doc, "iterations", NITERATIONS);
    let concurrency = read_i64_configuration(config_doc, "concurrency", iterations);
    let rampup = read_i64_configuration(config_doc, "rampup", NRAMPUP);
    let base = config_doc["base"].as_str().unwrap().to_owned();

    if concurrency > iterations {
      panic!("The concurrency can not be higher than the number of iterations")
    }

    Config {
      base,
      concurrency,
      iterations,
      relaxed_interpolations,
      no_check_certificate,
      rampup,
      quiet,
      nanosec,
    }
  }
}

fn read_i64_configuration(config_doc: &Yaml, name: &str, default: i64) -> i64 {
  match config_doc[name].as_i64() {
    Some(value) => {
      if value < 0 {
        println!("Invalid negative {} value!", name);

        default
      } else {
        value
      }
    }
    None => {
      if config_doc[name].as_str().is_some() {
        println!("Invalid {} value!", name);
      }

      default
    }
  }
}
