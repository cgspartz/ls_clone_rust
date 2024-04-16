use std::{env, fs};
use std::path::{Path,PathBuf};
use std::error::Error;
use std::process;
use chrono::{DateTime, Local};
// Examples:
// * `S_IRGRP` stands for "read permission for group",
// * `S_IXUSR` stands for "execution permission for user"
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::PermissionsExt;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let mut path = PathBuf::new();
	if args.len() > 0 {
		path.push(&args[0]);
	} else {
		path.push(".");
	}
	if let Err(ref e) = run(&path) {
		println!("{}", e);
		process::exit(1);
	}
}

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				let metadata = entry.metadata()?;
				let size = metadata.len();
				let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
				let mode = metadata.permissions().mode();
				println!("{0: <10} {1:<10} {2: <10} {3: <10}",
						 parse_permissions(mode as u16),
						 parse_size(size),
						 modified.format("%_d %b %H:%M").to_string(),
						 file_name
				);
		}
	}
	Ok(())
}

fn parse_size(size: u64) -> String {
	let length = size.checked_ilog10().unwrap_or(0) + 1;
	let mut res = size.to_string();
	if length > 3 { 
		if length%3==2 {
			res.insert(2, '.');
		} else if length%3==1 {
			res.insert(1, '.');
		} else {
			res.insert(3, '.');
		}
	}
	[res,bytes_symbol(length)].join("")
}

fn bytes_symbol(length: u32) -> String {
	match length {
		1..=3 => "B",
		4..=6 => "kB",
		7..=9 => "mB",
		10..=12 => "GB",
		_ => ""
	}.to_string()
}

fn parse_permissions(mode: u16) -> String {
	let user = triplet(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
	let group = triplet(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
	let other = triplet(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);
	[user, group, other].join("")
}

fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
	match (mode & read, mode & write, mode & execute) {
		(0, 0, 0) => "---",
		(_, 0, 0) => "r--",
		(0, _, 0) => "-w-",
		(0, 0, _) => "--x",
		(_, 0, _) => "r-x",
		(_, _, 0) => "rw-",
		(0, _, _) => "-wx",
		(_, _, _) => "rwx",
	}.to_string()
}


