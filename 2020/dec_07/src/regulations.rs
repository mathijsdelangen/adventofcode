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
                let mut contents = allowed_contents.to_owned();
                contents.dedup();
                if contents.contains(&bag_color) {result = true}
                else {
                    for content in contents {
                        result = result || self.can_contain(content.to_string(), bag_color.to_string());
                    }
                }
            },
            None => ()
        }
        
        return result;
    }

    pub fn contains_nr_bags(&self, bag: String) -> usize {
        let mut count = 0;
        
        match self.bag_contents.get(&bag) {
            Some(allowed_contents) => 
            { 
                //println!("{} contains {} bags", bag, allowed_contents.len());
                for content in allowed_contents {
                    count += self.contains_nr_bags(content.to_string());
                }
                count += allowed_contents.len();
            }
            None => (),
        }
        
        return count;
    }
}
