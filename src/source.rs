use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SourceFile {
    path: PathBuf,
    content: String,
}

impl SourceFile {
    
    pub fn load<P : AsRef<Path>>( path : P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();        
        let content = fs::read_to_string(&path)?;
        Ok(Self {path, content})
    }

    pub fn name(&self) -> String {
        self.path.display().to_string()
    }    

    pub fn content(&self) -> &str {
        &self.content
    }

}


pub fn load_source_files(paths : &[impl AsRef<Path>]) -> Result<Vec<SourceFile>, SourceFileErrors> {
    let mut sources = Vec::new();
    let mut errors = Vec::new();
    
    for path in paths {
        match SourceFile::load(path) {
            Ok(src) => sources.push(src),
            Err(e) => errors.push(SourceFileError(path.as_ref().to_path_buf(), e)),
        }
    }
    
    if errors.is_empty() {
        Ok(sources)
    } else {
        Err(SourceFileErrors {errors } )
    }
}


#[derive(Debug)]
pub struct SourceFileError(PathBuf, io::Error);

impl std::fmt::Display for SourceFileError {

    fn fmt(&self, f :&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`\n    {}",  self.0.display(), self.1)
    }

}

impl std::error::Error for SourceFileError {}

#[derive(Debug)]
pub struct SourceFileErrors {
    errors: Vec<SourceFileError>,
}

impl std::fmt::Display for SourceFileErrors {

    fn fmt(&self, f :&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to load {} file(s):", self.errors.len())?;
        for (n, error) in self.errors.iter().enumerate() {
            write!(f, "\n {} - {}", n + 1, error)?;
        }
        Ok(())
    }

}

impl std::error::Error for SourceFileErrors {}


#[cfg(test)]
mod tests {}
