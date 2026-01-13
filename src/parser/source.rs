use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct SourceIterator {
    files: Vec<PathBuf>,
    current_reader: Option<BufReader<File>>,
    current_index: usize,
    buffer : [u8,4],
    buffer_len : usize,
}

impl SourceIterator {
    pub fn new(files: Vec<PathBuf>) -> Self {
       
        Self {files, 
            current_reader : None,
            current_index: 0,
            buffer: [0; 4],
            
        }    
    }
}


impl Iterator for SourceIterator {
    type Item = Result<&str, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_file == self.file_count {
            return None;
        }

        let file = self.files[self.current_file];
        self.contents = fs::read_to_string(file)?;
        self.current_file += 1;
        Some(Ok(&self.contents))
    }
}