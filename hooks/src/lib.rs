pub struct RegistrationHooks {
    pub pre_register: String,
    pub post_register: String,
}

impl RegistrationHooks {
    pub fn new() -> RegistrationHooks {
        RegistrationHooks {
            pre_register: String::new(),
            post_register: String::new(),
        }
    }

    pub async fn exec_pre_register(&self) {
        //self.pre_register
    }

    pub async fn exec_post_register(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
