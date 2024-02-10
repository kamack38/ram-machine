use crate::parser::CodeParseError;

#[derive(Debug)]
pub struct ParserErrorChain(Vec<(u32, CodeParseError)>);

impl ParserErrorChain {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, (line, err): (u32, CodeParseError)) {
        self.0.push((line, err))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for ParserErrorChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output: String = "".to_string();
        for (line, err) in &self.0 {
            output.push_str(&format!(
                "At line {} found error: {}\n",
                line.to_string(),
                err
            ));
        }
        write!(f, "{}", output)
    }
}
