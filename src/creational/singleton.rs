use lazy_static::lazy_static;
pub struct Singleton {
    data: Option<i32>
}

impl Singleton {
    fn new() -> Singleton {
        Singleton { data: Some(0i32) }
    }

    pub fn get_data(&self) -> Option<i32> {
        self.data
    }
}

lazy_static! {
    static ref SINGLETON: Singleton = Singleton::new();
}

pub fn get_instance() -> &'static Singleton {
    &SINGLETON
}