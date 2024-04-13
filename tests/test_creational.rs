
#[cfg(test)]
mod tests {
 //   use super::*;

    use design_patterns::creational::singleton;
    #[test]
    fn it_works() {
        // 获取单例对象的引用
    let singleton_ref = singleton::LazySingleton::instance();
    assert_eq!(singleton_ref.lock().unwrap().get_data().to_string(),"Lazy Singleton Instance" );
    }
}