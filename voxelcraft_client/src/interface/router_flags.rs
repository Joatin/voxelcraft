use crate::interface::page::Page;
use std::collections::HashMap;

pub struct RouterFlags {
    pub pages: HashMap<String, Box<dyn Page>>,
    pub initial_route: String,
}
