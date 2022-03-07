use std::collections::HashMap;
use crate::interface::page::Page;

pub struct RouterFlags {
    pub pages: HashMap<String, Box<dyn Page>>,
    pub initial_route: String,
}