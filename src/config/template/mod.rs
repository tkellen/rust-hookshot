use std::collections::HashMap;
use handlebars::Handlebars;

pub type Context = HashMap<String, String>;

pub struct Template {
    file_path: &Path,
    handlebars: Handlebars
}

impl Template {
    fn new(template: String) -> Template {
        let mut hb = Handlebars::new();
        let _ = hb.register_template_string("template", template);
        Template {
            handlebars: hb
        }
    }

    pub fn render(&self, data: &Context) -> String {
        self.handlebars.render("template", data).unwrap()
    }
}
