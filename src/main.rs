
struct Car {
    color: String,
    engine: Engine,
    wheels: u32,
}

struct Engine {
    size: u32,
}

struct CarBuilder {
    color: Option<String>,
    engine: Option<Engine>,
    wheels: Option<u32>,
}

impl CarBuilder {
    fn new() -> Self {
        Self {
            color: None,
            engine: None,
            wheels: None,
        }
    }

    fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
    fn engine(mut self, engine: Engine) -> Self {
        self.engine= Some(engine);
        self
    }
    fn wheels(mut self, wheels: u32) -> Self {
        self.wheels= Some(wheels);
        self
    }

    fn build(self)-> Result<Car,String> {
        let color = self.color.ok_or("Color is required")?;
        let engine = self.engine.ok_or("Engine is required")?;
        let wheels = self.wheels.ok_or("Wheels is required")?;
        Ok(Car{
            color,
            engine,
            wheels,
        })
    }
}

fn main() {
    // 使用建造者模式创建一个 Car 实例
    let car = CarBuilder::new()
        //.color("Red".to_string())
        .engine(Engine { size: 2 })
        .wheels(4)
        .build();

        match car {
            Ok(obj) => println!("Car color: {}", obj.color),
            Err(err) => println!("Error: {}", err),
        }
}
