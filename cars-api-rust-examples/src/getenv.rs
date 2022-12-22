/*
A library to get OS environment variables
I wrote this because I am coming from Python and I wanted something similar to os.getenv() except I want the Rust program to fail if the environment variable is not set.
*/
use std::env;

pub fn get_os_env_var(var_name: &str) -> String {
    let value = match env::var(var_name) {
        Ok(val) => val,
        Err(e) => panic!("Environment variable {} not set: {}", var_name, e),
    };
    return value;
}
