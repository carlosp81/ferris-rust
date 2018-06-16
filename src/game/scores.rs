// Copyright © 2018
// "River Bartz"<bpg@pdx.edu>
// "Daniel Dupriest"<kououken@gmail.com>
// "Brandon Goldbeck"<rbartz@pdx.edu>
// This program is licensed under the "MIT License". Please see the file
// LICENSE in the source distribution of this software for license terms.

//use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

const MAX_SCORES: usize = 10;

/// Loads and saves scores in an external file.
pub struct Scores {
	scores: Vec<(i32, String, String)>,
}

impl Scores {
	/// Create a new scores object.
    pub fn new(file: &str) -> Scores {        
        let mut f = match OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.open(file) {
			Ok(f) => f,
			Err(e) => panic!(e),
		};

		let mut scores = Vec::new();

		let mut file_contents = String::new();
		match f.read_to_string(&mut file_contents) {
			Ok(_o) => (),
			Err(e) => panic!(e),
		}
		let lines: Vec<&str> = file_contents.split("\n").collect();
		for i in 0 .. lines.len()-1 {
			let values: Vec<&str> = lines[i].split("|").collect();
			println!("length = {}, {:?}", values.len(), values);
		
			scores.push( (
				values[0].parse::<i32>().unwrap(),
				values[1].to_string(),
				values[2].to_string()
			) );
		}
		
		Scores {
			scores: scores,
		}
    }

	/// Adds a score to the list
    pub fn add_score(&mut self, number: i32, name: String, time: String) {
        self.scores.push( (number, name, time) );
		self.scores.sort_by(|a,b| b.0.cmp(&a.0));
		while self.scores.len() > MAX_SCORES {
			self.scores.remove(MAX_SCORES);
		}
	}

	/// Returns a vector of tuples with score, name and time
    pub fn get_scores(&self) -> &Vec<(i32, String, String)> {
        &self.scores
    }
	
	/// Saves scores to file
	pub fn save(&self, file: &str) {
        let mut f = match OpenOptions::new()
			.write(true)
			.create(true)
			.open(file) {
			Ok(f) => f,
			Err(e) => panic!(e),
		};

		for (score, name, time) in &self.scores {
			let combined = format!("{}|{}|{}\n", score, name, time);
			match f.write(combined.as_bytes()) {
				Ok(_o) => (),
				Err(e) => panic!(e),
			}
		}
	}
}