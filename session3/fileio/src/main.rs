use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file =  File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let now = std::time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) =  read_lines("../warandpeace.txt"){
        lines.for_each(|line|{
            if let Ok(line) = line{
                if !line.trim().is_empty(){
                    line_count +=1;
                }
            }
        });
    }

    println!("Read {} lines in {:?} seconds", line_count, now.elapsed().as_secs_f32());
}

