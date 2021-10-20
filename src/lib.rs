
pub use loadenv_derive::EnvLoad;


pub struct EnvValue {
    raw: String,
}

impl EnvValue {
    pub fn new(raw: String) -> Self {
        Self {raw}
    }
}


impl Into<String> for EnvValue {
    fn into(self) -> String {
        self.raw
    }
}

impl Into<u32>  for EnvValue {
    fn into(self) -> u32 {
        self.raw.parse().unwrap()
    }
}


#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        #[derive(loadenv::EnvLoad)]
        struct Conf {
            #[env("VAR_STR", "XXXX")] 
            pub varstr: String,
            #[env("VAR_U32", 32u32)]
            pub varu32: u32,
        }

        let mut conf = Conf {varstr: "".into(), varu32: 0};

        conf.load_env();
        assert_eq!("XXXX", conf.varstr);
        assert_eq!(32u32, conf.varu32);

        std::env::set_var("VAR_STR", "Hello");
        std::env::set_var("VAR_U32", "111");

        conf.load_env();
        assert_eq!("Hello", conf.varstr);
        assert_eq!(111u32, conf.varu32);

    }
}


