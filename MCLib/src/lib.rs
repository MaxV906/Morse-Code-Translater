// TO DO:
// Implement binary tree for morse code to ascii
// Implement morse code to ascii function in the Morse Translater
// Implement tests for morse to ascii translation

pub mod Binary_Tree {
    
    pub struct Node {
        pub value: Option<char>,
        pub left: Option<Box<Node>>,
        pub right: Option<Box<Node>>
    }

    impl Node {

        pub fn new(value: Option<char>) -> Node {
            Node {
                value,
                left: None,
                right: None
            }
        }

        pub fn insert_left(&mut self, value: Option<char>) {

            if self.left.is_some() {
                return
            }
            
            self.left = Some(Box::new(Node::new(value)));
            
        }

        pub fn insert_right(&mut self, value: Option<char>) {

            if self.right.is_some() {
                return
            }
            
            self.right = Some(Box::new(Node::new(value)));
            
        }
        
    }

}

pub enum Translation {
    ASCII,
    CODE
}

pub enum Error {
    MessageNotSet,
    NotValidChar
}

pub struct Morse {
    code: Vec<String>,
    tree: crate::Binary_Tree::Node,
    message: Option<String>
}

impl Morse {
   
    // Morse code translater constructor
    pub fn new() -> Morse {

        Morse {
            code: Morse::make_vector(), // Inserts ascii to code vector
            tree: crate::Binary_Tree::Node::new(None), // Temporarily makes a empty binary tree node
            message: None // Sets the message to none
        }

    }

    // Sets the message for translation
    pub fn set_message(&mut self, message: &str) {

        self.message = Some(String::from(message));

    }

    // Translates and returns a result
    pub fn translate(&self, translation: Translation) -> Result<String, Error> {

        match translation {
            Translation::ASCII => {
                return self.translate_to_code(); // Result from ascii to code translation
            },
            Translation::CODE => {return Ok("Test".to_string())}, // Will be implemented properly later
        }

    }

    // Translates ascii to code
    fn translate_to_code(&self) -> Result<String, Error> {

        if self.message.is_none() {
            return Err(Error::MessageNotSet);
        }
        
        let binding = self.message.as_ref().unwrap().clone().to_uppercase();
        let message = binding.chars();
        let mut translation: String = String::new();

        for c in message {

            let c_code = c as usize;
            let i = c_code - 65;

            if c.is_alphabetic() {
                translation.push_str(format!("{} ", self.code[i]).as_str());
                continue;
            }

            if c.is_whitespace() {
                if c != ' ' {
                    return Err(Error::NotValidChar)
                }

                translation.push_str("   ");
                continue;
            }

            return Err(Error::NotValidChar);

        }

        let _ = translation.pop();
        
        return Ok(translation);
        
    }

    fn make_vector() -> Vec<String> {

        // Creates a vector containing morse code for each character

        return vec![
            String::from(".-"),
            String::from("-..."),
            String::from("-.-."),
            String::from("-.."),
            String::from("."),
            String::from("..-."),
            String::from("--."),
            String::from("...."),
            String::from(".."),
            String::from(".---"),
            String::from("-.-"),
            String::from(".-.."),
            String::from("--"),
            String::from("-."),
            String::from("---"),
            String::from(".--."),
            String::from("--.-"),
            String::from(".-."),
            String::from("..."),
            String::from("-"),
            String::from("..-"),
            String::from("...-"),
            String::from(".--"),
            String::from("-..-"),
            String::from("-.--"),
            String::from("--..")
        ];

    }
 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_to_code_test() {
        let mut mst: Morse = Morse::new();
        mst.set_message("SOS");

        match mst.translate(Translation::ASCII) {
            Ok(res) => {
                assert_eq!(res, String::from("... --- ..."));
            },

            Err(_) => {
                assert!(false);
            }
        }
    }
    
}
