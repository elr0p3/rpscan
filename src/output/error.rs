use std::fmt;
use std::error;


#[derive(Debug)]
pub struct OutfileError {
    kind: OutfileErrorKind,
}

#[derive(Debug)]
pub enum OutfileErrorKind {
    FileAlreadyExists,
    DirectoryAlreadyExists,
    Skip,
    InvalidMode,
}

impl OutfileError {

    pub fn new (kind: OutfileErrorKind) -> Self {
        Self {
            kind,
        }
    }

    pub fn kind (&self) -> &OutfileErrorKind {
        &self.kind
    }

    pub fn __description (&self) -> &str {
        match self.kind {
            OutfileErrorKind::FileAlreadyExists =>
                "cannot create this file because it already exists",
            OutfileErrorKind::DirectoryAlreadyExists =>
                "cannot create this file because it is a directory",
            OutfileErrorKind::Skip => 
                "do not store results in a file",
            OutfileErrorKind::InvalidMode =>
                "invalid storing mode selected",
        }
    }
}

impl fmt::Display for OutfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
        // write!(f, "")
    }
}

impl error::Error for OutfileError {
    // fn source (&self) -> Option<&(dyn Error + 'static)> {
        // Some(&self.kind)
    // }
}
