
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    
        #[test]
        fn test_area(){
            let rect = Rectangle {
                width: 30,
                height: 50,
            };
            assert_eq!(rect.area(), 1500);
        }
    
        #[test]
        fn test_can_hold(){
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            let rect2 = Rectangle {
                width: 10,
                height: 40,
            };
            let rect3 = Rectangle {
                width: 60,
                height: 45,
            };
            assert!(rect1.can_hold(&rect2));
            assert!(!rect1.can_hold(&rect3));
        }

        #[test]
        fn test_no_hold(){
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            let rect2 = Rectangle {
                width: 60,
                height: 45,
            };
            assert!(!rect1.can_hold(&rect2));
            assert!(!rect2.can_hold(&rect1));
        
        }
    
        #[test]
        fn test_square(){
            let square = Rectangle::square(10);
            assert_eq!(square.width, 10);
            assert_eq!(square.height, 10);
        }
}


mod more_test{
    #[test]
        fn check(){
            assert_eq!(2+2, 4);
        }
    
}