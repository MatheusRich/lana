#[derive(PartialEq, Debug, Clone)]
pub struct SrcLocation {
    pub line: i32,
    pub col: i32,
}

impl SrcLocation {
    pub fn default() -> Self {
        SrcLocation { line: 1, col: 0 }
    }

    #[cfg(test)]
    pub fn new(line: i32, col: i32) -> Self {
        SrcLocation { line, col }
    }
}

impl std::fmt::Display for SrcLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.col)
    }
}
