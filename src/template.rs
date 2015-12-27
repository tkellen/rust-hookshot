use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

use handlebars::Handlebars;

pub type Data = HashMap<String, String>;

pub struct Template {
    handlebars: Handlebars,
}

impl Template {
    fn new(template: String) -> Template {
        let mut hb = Handlebars::new();
        let _ = hb.register_template_string("template", template);
        Template { handlebars: hb }
    }

    pub fn render(&self, data: &Data) -> String {
        self.handlebars.render("template", data).unwrap()
    }
}

fn read_file(filepath: &str) -> Result<String, String> {
    let path = Path::new(filepath);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => return Err(format!("Couldn't open {}: {}", display, Error::description(&e))),
        Ok(file) => file
    };
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
      return Err(format!("Couldn't read {}: {}", display, Error::description(&e)));
    }
    Ok(content)
}


pub fn load(filepath: &str) -> Result<Template, String> {
    match read_file(filepath) {
        Ok(contents) => Ok(Template::new(contents)),
        Err(err) => Err(err)
    }
}
