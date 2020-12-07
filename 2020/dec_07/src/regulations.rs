use std::collections::HashMap;

pub struct Regulations {
    known_bags: Vec<String>,
    bag_contents: HashMap<String, Vec<String>>
}

impl Regulations {
    pub fn new() -> Regulations {
        Regulations { known_bags:vec![], bag_contents:HashMap::new()}
    }

    pub fn get_known_bags(&self) -> Vec<String> {
        return self.known_bags.to_owned();
    }

    pub fn add_bag(&mut self, bag_color: &str) {
        //println!("Add bag '{}'", bag_color);
        self.known_bags.push(String::from(bag_color));
        self.bag_contents.insert(String::from(bag_color), vec![]);
    }

    pub fn add_bag_content(&mut self, bag_color: &str, content: &str) {
        //println!("Add '{}' to '{}'", content, bag_color);
        self.bag_contents.get_mut(&String::from(bag_color)).unwrap().push(String::from(content));
    }

    pub fn can_contain(&self, bag: String, bag_color: String) -> bool {
        let mut result = false;
        match self.bag_contents.get(&bag) {
            Some(allowed_contents) => 
            { 
                if allowed_contents.contains(&bag_color) {result = true}
                else {
                    for content in allowed_contents {
                        result = result || self.can_contain(content.to_string(), bag_color.to_string());
                    }
                }
            },
            None => ()
        }
        
        return result;
    }
}