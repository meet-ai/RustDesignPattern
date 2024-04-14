

#[cfg(test)]
mod tests {
    //   use super::*;

    use design_patterns::creational::singleton;
    #[test]
    fn it_works() {
        // 获取单例对象的引用
        let instance = singleton::get_instance();
        println!("{}",instance.get_data().unwrap());
    }
}
