use std::{
    env,
    fmt::write,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    io::{prelude::*, BufWriter},
    usize,
};

fn main() {
    // ----------Files Read----------
    let filepath = env::args().nth(1).expect("please provide filepath");
    let _ = read_file3(&filepath);

    // ----------Files Write----------
    let text = "hello world";
    let _ = write_file2(&filepath, text);
}

fn write_file(filepath: &str, text: &str) -> io::Result<bool> {
    let mut file = File::create(filepath)?;
    write!(file, "123")?;
    Ok(true)
}

fn write_file2(filepath: &str, text: &str) -> io::Result<usize> {
    let file = File::create(filepath)?;
    let mut writer = BufWriter::new(file);
    let usize = writer.write(text.as_bytes())?;
    Ok(usize)
}

fn read_file(filepath: &str) -> io::Result<()> {
    let file = File::open(filepath)?;

    let reader = BufReader::new(file);
    // let buf = String::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        println!("read1({idx}): {line}");
    }
    Ok(())
}

fn read_file2(filepath: &str) -> io::Result<()> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut idx = 0;
    while reader.read_line(&mut buf)? > 0 {
        let line = buf.trim();
        println!("read2({idx}): {line}");
        idx += 1;
        buf.clear();
        // println!("read({idx}): {line}");
    }
    Ok(())
}

fn read_file3(filepath: &str) -> io::Result<()> {
    let file = File::open(filepath)?;
    let mut lines = Lines::new(file);

    while let Some(Ok(text)) = lines.next() {
        println!("read3: {text}");
    }

    Ok(())
}

struct Lines<R> {
    reader: BufReader<R>,
    buf: String,
}

impl<R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines {
            reader: BufReader::new(r),
            buf: String::new(),
        }
    }
    fn next(&mut self) -> Option<io::Result<&str>> {
        self.buf.clear();

        match self.reader.read_line(&mut self.buf) {
            Ok(nsize) => {
                if nsize == 0 {
                    None
                } else {
                    let line = self.buf.trim();
                    Some(Ok(line))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}
