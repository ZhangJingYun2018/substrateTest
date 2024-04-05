fn main() {

    //为 enum 交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

    trait TrafficLight {
        fn get_time(&self) -> u32;
    }

    enum TrafficLightEnum {
        Red,
        Yellow,
        Green,
        
    }


    impl TrafficLight for TrafficLightEnum {
        fn get_time(&self) -> u32 {
            match self {
                TrafficLightEnum::Red => 5,
                TrafficLightEnum::Yellow => 2,
                TrafficLightEnum::Green => 3,
            }
        }
    }

    //实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None

    fn sum(numbers: &[u32]) -> Option<u32> {
        let sum:u32 = numbers.iter().sum();
        if sum > u32::MAX {
            return None;
        }
        Some(sum)
    }

   // 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

    fn print_area<T: Shape>(shape: T) {
        println!("The area is: {}", shape.area());
    }

    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    struct Triangle {
        base: f64,
        height: f64,
    }

    impl Shape for Triangle {
        fn area(&self) -> f64 {
            (self.base * self.height) / 2.0
        }
    }

    struct Square {
        side: f64,
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }
  
}
