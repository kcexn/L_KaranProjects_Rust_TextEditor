
pub struct TextData {
    input: String,
    options: TextDataOptions,
    text: Vec<String>,
}

impl TextData {
    pub fn new(input_str: &str, text: Vec<String>) -> TextData {
        return TextData {
            input: String::from(input_str),
            options: TextDataOptions::new(0),
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

    pub fn set_write_head(&mut self, line_no: usize) {
        assert!(line_no > 0);
        self.options.line_no = line_no;
    }

    fn get_write_head(&self) -> usize {
        return self.options.line_no;
    }

    pub fn print_write_line(&self, line_no: usize) {
        print!("{}", self.text[line_no]); 
    }

    pub fn edit_line(&mut self, edit_line: &str) {
        let line_no = self.get_write_head();
        self.text[line_no] = String::from(edit_line);
    }
}

struct TextDataOptions {
   line_no: usize,     
}

impl TextDataOptions {
    fn new(line_no: usize) -> TextDataOptions {
        return TextDataOptions {
            line_no
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
