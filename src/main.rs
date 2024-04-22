use std::path::Path;
use std::process;
mod cli_parse;
use cli_parse::Value;
mod run_ls;

fn main() {
	let matches: clap::ArgMatches = cli_parse::cli().get_matches();
	let values = Value::from_matches(&matches);
	let mut dir= ".";
	let mut hiddenfiles = &false;
	let mut humansize = &false;
	for (id,value) in values.iter() {
		if id.as_str() == "directory" {
			match value {
				Value::String(value) => dir = value,
				_ => println!("Thats not right"),
			}
		} else if id.as_str() == "all" {
			hiddenfiles = value_bool(value);
		} else if id.as_str() == "size" {
			humansize = value_bool(value);
		}
	}
	let path = Path::new(dir);
	if let Err(ref e) = run_ls::run(&path, hiddenfiles, humansize) {
		println!("{}", e);
		process::exit(1);
	}
}

fn value_bool(val:&Value) -> &bool {
	let res;
	match val {
		Value::Bool(value) => res = value,
		_ => res = &false,
	}
	res
}