use std::fs::File;
use std::io;
use std::path::PathBuf;

// manages source code
pub struct Source {
    files: Vec<PathBuf>,
    reader: 
    current_column : usize, 
    current_line : usize, 
    current_index : usize,
    contents: String,
}

impl SourceIterator<'source> {
    pub fn new(files: Vec<PathBuf>) -> Self {
       
        Self {
            files,
            reader 
            current_file : 0,
            contents : String::new(),
        }    
    }

    pub fn open_next_file(&mut self) -> io::Result<bool> {
        if self.current_file >= self.files.len() {
            return Ok(false);
        }

        let file = File::open(&self.files[self.current_index])?;
        f.read_to_string(self.contents)?;
        self.current_index += 1;
        Ok(true)
    }

    pub fn get_source(&'source self) -> &'source str {
        &self.contents
    }
    
}


impl Iterator for SourceIterator {
    type Item = std::io::Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_reader.is_none() {
                match self.open_next_file() {
                    Ok(true) => {},
                    Ok(false) => return None,
                    Err(e) => return Some(Err(e)),
                }
            }

            let reader = self.current_reader.as_mut().unwrap();

            // read one byte 
            let mut byte = [0u8; 1];
            match reader.read(&mut byte) {
                Ok(0) => {
                    // EOF on current file, move to next

                    self.current_reader = None;
                    continue;
                            
                }
                Ok(_) => {
                    self.buffer[self.buffer_len] = byte[0];
                    self.buffer_len += 1;

                    match std::str::from_utf8(&self.buffer[..self.buffer_len]) {
                        Ok(s) => {
                            let ch = s.chars().next().unwrap();
                            self.buffer_len = 0;
                            return Some(Ok(ch));
                        }
                        Err(e) if self.buffer_len < 4 && e.error_len().is_none() => {
                            continue;
                        }
                        Err(_) => {
                            self.buffer_len = 0;
                            return Some(Self::invalid_utf8());
                        }
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}