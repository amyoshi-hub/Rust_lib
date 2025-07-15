# simple file handler

A minimal file handling utility written in Rust
You can easily read and write text files, and extract numeric values using customizable separators.

# main code
```rs
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
      
```

# planning features
- better error handling
- json, csv support
- cli support
- library export
