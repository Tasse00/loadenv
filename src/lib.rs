
pub use loadenv_derive::LoadEnv;


pub struct EnvValue {
    raw: String,
}



impl EnvValue {
    pub fn new(raw: String) -> Self {
        Self {raw}
    }
}
macro_rules! convert_user_parse {
    ($type:ty) => {    
        impl Into<$type> for EnvValue {
            fn into(self) -> $type {
                self.raw.parse().unwrap()
            }
        }
    };
}

convert_user_parse!(f32);
convert_user_parse!(f64);
convert_user_parse!(bool);
convert_user_parse!(String);
convert_user_parse!(u16);
convert_user_parse!(u32);
convert_user_parse!(u64);
convert_user_parse!(u128);

#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        #[derive(loadenv::LoadEnv)]
        struct Conf {
            #[env("VAR_STR", "XXXX")] 
            pub varstr: String,
            #[env("VAR_BOOL", false)]
            pub varbool: bool,
            #[env("VAR_U16", 16u16)]
            pub varu16: u16,
            #[env("VAR_U32", 32u32)]
            pub varu32: u32,
            #[env("VAR_U64", 64u64)]
            pub varu64: u64,
            #[env("VAR_U128", 128u128)]
            pub varu128: u128,
            #[env("VAR_F32", 32f32)]
            pub varf32: f32,
            #[env("VAR_F64", 64f64)]
            pub varf64: f64,
            
        }

        let mut conf = Conf {
            varstr: "".into(), 
            varu16: 0,
            varu32: 0,
            varu64: 0,
            varu128: 0,
            varf32: 0.0,
            varf64: 0.0,
            varbool: true,
        };

        conf.load_env();
        assert_eq!("XXXX", conf.varstr);
        assert_eq!(16u16, conf.varu16);
        assert_eq!(32u32, conf.varu32);
        assert_eq!(64u64, conf.varu64);
        assert_eq!(128u128, conf.varu128);
        assert_eq!(32f32, conf.varf32);
        assert_eq!(64f64, conf.varf64);
        assert_eq!(false, conf.varbool);

        std::env::set_var("VAR_STR", "Hello");
        std::env::set_var("VAR_U16", "160");
        std::env::set_var("VAR_U32", "320");
        std::env::set_var("VAR_U64", "640");
        std::env::set_var("VAR_U128", "1280");
        std::env::set_var("VAR_F32", "32.32");
        std::env::set_var("VAR_F64", "64.64");
        std::env::set_var("VAR_BOOL", "true");

        conf.load_env();
        assert_eq!("Hello", conf.varstr);
        assert_eq!(160, conf.varu16);
        assert_eq!(320, conf.varu32);
        assert_eq!(640, conf.varu64);
        assert_eq!(1280, conf.varu128);
        assert_eq!(32.32, conf.varf32);
        assert_eq!(64.64, conf.varf64);
        assert_eq!(true, conf.varbool);
        

    }
}


