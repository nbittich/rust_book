mod chapter11 {
    #[derive(Debug)]
    pub struct Rectangle {
        pub width:u32,
        pub height:u32
    }

    impl Rectangle {
        pub fn can_handle(&self, other: &Rectangle) -> bool {
            self.height > other.height && self.width > other.width
        }
    }

    pub fn add_two(val: &u32) -> u32{
        val + 2
    }
}

#[cfg(test)]
mod tests {
    use super::chapter11::*;
    #[test]
    fn it_can_hold() {
        let r1 = Rectangle{
            width: 10,
            height: 12
        };
        let r2 = Rectangle{
            width: 5,
            height: 6
        };

        assert!(r1.can_handle(&r2));
    }

    #[test]
    fn add_two_test() {
        assert_eq!(4, add_two(&2));
        assert_ne!(5, add_two(&2));
    }
}
