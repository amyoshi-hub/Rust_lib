use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use regex::Regex;

pub struct FileIO{
    pub filename: String,
    pub contents: Vec<String>,
}

impl FileIO {
    pub fn new(filename: &str) -> Self{
        Self {
            filename: filename.to_string(),
            contents: Vec::new(),
        }
    }

    fn read_lines(&mut self) -> io::Result<()>{
        let file = File::open(&self.filename)?;
        let reader = BufReader::new(file);
        self.contents = reader
            .lines()
            .map(|l| l.unwrap_or_default())
            .collect();
            Ok(())
    }

    pub fn print_lines(&self){
        for line in &self.contents {
            println!("{}", line);
        }
    }

    pub fn read(&mut self) -> io::Result<()>{
       self.read_lines()?; 
       self.print_lines();
       Ok(())
    }

    pub fn phaser(contents: &str, phase_index: &[&str]) -> Vec<String>{
        let pattern = phase_index.join("|");
        let re = Regex::new(&pattern).unwrap();
        re.split(contents).map(|s| s.to_string()).collect()
    }

    pub fn write_file(filename: &str, lines: &[String]) -> io::Result<()>{
        let mut file = File::create(filename)?;
        for line in lines{
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut file = FileIO::new("word_list.txt");
    file.read_lines()?;
    
    let sep_list = [":", "|"];
    let mut result = Vec::new();

    for line in &file.contents {
        let split = FileIO::phaser(line, &sep_list);
        let filtered: Vec<_> = split
            .iter()
            .filter(|s| s.trim().parse::<f32>().is_ok())
            .cloned()
            .collect();
        let joined = filtered.join(", ");
        result.push(joined);
    }

    FileIO::write_file("out.txt", &result)?;
    
    Ok(())
}
