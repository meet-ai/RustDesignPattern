// 定义一个包含单例对象的模块
use std::sync::{ Arc, Mutex };

// 定义一个懒加载单例结构体
pub struct LazySingleton {
    data: String,
}

//
impl LazySingleton {
    fn _new() -> LazySingleton {
        LazySingleton {
            data: String::from("Lazy Singleton Instance"),
        }
    }

    // 公共获取单例实例的方法
    pub fn instance() -> Arc<Mutex<LazySingleton>> {
        static INSTANCE: once_cell::sync::Lazy<Arc<Mutex<LazySingleton>>> = once_cell::sync::Lazy::new(
            || Arc::new(Mutex::new(LazySingleton::_new()))
        );
        INSTANCE.clone()
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
}
