use super::base;

pub struct Environment {
    main_character: Box<base::BaseObject>,
    objects: Vec<Box<base::BaseObject>>
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            main_character: Box::new(base::Arrow { 
                name: String::from("Black Arrow"),
                description: String::from("Does not seem to be a main character of nothing")
            }),
            objects: vec!(Box::new(base::Arrow { 
                name: String::from("Main Door"),
                description: String::from("It's just a main door")
            }))
        }
    }

    pub fn find_object_by_name(&self, name: &str) -> Option<&Box<base::BaseObject>> {
        let cmp_name = name.to_lowercase();
        self.objects.iter().find(|o| o.name().to_lowercase() == cmp_name)
    }

}