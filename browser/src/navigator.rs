/// This is here in preparation for when navigating web pages become more complex

pub struct Navigator {
    pub current_loc: String,
}

impl Navigator {
    fn nav_forward() {}

    fn nav_backward() {}

    fn nav_home(&mut self) -> String {
        let resp = reqwest::blocking::get(&self.current_loc)
            .unwrap()
            .text()
            .unwrap();
        return resp;
    }

    fn set_current_loc(&mut self, new_loc: String) {
        self.current_loc = new_loc;
    }
}
