use std::fs::File;
use std::io::prelude::*;

use handlebars::Handlebars;
use serde::ser::Serialize;

pub struct ResourceManager {
    pages: Handlebars,
}

impl ResourceManager {
    pub fn new(page_names: &[&'static str], partials: &[&'static str]) -> ResourceManager {
        let mut pages = Handlebars::new();
        for partial_name in partials {
            let path = format!("www/{}.partial.html", partial_name);
            let page = ResourceManager::read_resource_from_disk(&path);
            pages.register_partial(partial_name, page).unwrap();
        }
        for page_name in page_names {
            let path = format!("www/{}.html", page_name);
            let page = ResourceManager::read_resource_from_disk(&path);
            pages.register_template_string(page_name, page).unwrap();
        }
        ResourceManager { pages }
    }

    pub fn get_page(&self, name: &'static str) -> String {
        self.render_page(name, ())
    }

    pub fn render_page<T: Serialize>(&self, name: &str, values: T) -> String {
        self.pages.render(name, &values).unwrap()
    }

    pub fn read_resource_from_disk(path: &str) -> String {
        let mut resource = String::new();
        info!("Reading resource {} into memory", path);
        let mut file = File::open(&path[..]).expect(&format!("Error opening file {}", path));
        file.read_to_string(&mut resource).expect(&format!(
            "Error reading {} from disk",
            path
        ));
        resource
    }
}
