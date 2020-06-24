#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn when_both_sides_of_other_are_smaller_than_self_then_self_can_hold_other() {
		let both_sides_bigger = Rectangle {
			width: 10,
			height: 5,
		};
		let both_sides_smaller = Rectangle {
			width: 8,
			height: 4,
		};
        assert!(both_sides_bigger.can_hold(&both_sides_smaller));
    }
	
	#[test]
    fn when_both_sides_of_other_are_smaller_than_self_then_other_cannot_hold_self() {
		let both_sides_bigger = Rectangle {
			width: 10,
			height: 5,
		};
		let both_sides_smaller = Rectangle {
			width: 8,
			height: 4,
		};
        assert!(!both_sides_smaller.can_hold(&both_sides_bigger));
    }
	
	#[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}