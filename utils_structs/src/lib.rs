pub struct TextData {
    input: String,
    text: Vec<String>,
}

impl TextData {
    pub fn new(input_str: &str, text: Vec<String>) -> TextData {
        return TextData {
            input: String::from(input_str),
            text
        }
    }

    pub fn set_input(&mut self, input_str: &str) {
       self.input = String::from(input_str);
    }

    pub fn get_input(&self) -> &str {
        return self.input.as_str();
    }

    pub fn append_to_file(&mut self) {
        assert!(self.input.len() > 0);
        self.text.push(String::from(self.input.as_str()));
        self.input = String::from("");
    }

    pub fn get_file(&self) -> &Vec<String> {
        return &self.text;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
