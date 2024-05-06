mod shapes {

    pub struct Circle {
        radius: i32,
    }

    impl Circle {
        pub fn new(radius: i32) -> Circle {
            Circle { radius }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }

        pub fn new_1(radius: i32) -> Result<Circle, String> {
            if radius > 0 {
                Ok(Self { radius })
            } else {
                Err("Radius not a positive number".to_string())
            }
        }

        pub fn new_2_panic(radius: i32) -> Circle {
            match radius {
                ..=0 => panic!("Invalid radius"),
                _ => Circle { radius },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use self::shapes::Circle; // super keyword is used to bring in all the modules in the crate

    use super::*;

    #[test]
    fn larger_circle_contains_smaller_circle() {
        let circle1 = shapes::Circle::new(5);
        let circle2 = shapes::Circle::new(2);

        assert_eq!(circle1.contains(&circle2), true); // method 1
        assert_ne!(circle1.contains(&circle2), false); // method 2
        assert!(circle1.contains(&circle2)); // method 3
    }

    #[test]
    fn smaller_circle_does_not_contains_larger_circle() {
        let circle1 = shapes::Circle::new(5);
        let circle2 = shapes::Circle::new(2);

        assert_eq!(circle2.contains(&circle1), false);
    }

    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        let circle1 = shapes::Circle::new_1(-1)?; // ? in the end returns error if execution encounters and error else control flow is passed to next line
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Invalid radius")]
    fn should_panic() {
        let circle1 = shapes::Circle::new_2_panic(-1);
    }

    #[test]
    #[ignore]
    fn test_large_execution() {}
}
