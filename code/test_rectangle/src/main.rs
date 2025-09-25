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

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "Can hold: {}",
        rect.can_hold(&Rectangle {
            width: 20,
            height: 30
        })
    );
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn assert() {
    assert!(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold() {
        let large = Rectangle {
            width: 2,
            height: 3,
        };
        let small = Rectangle {
            width: 1,
            height: 1,
        };
        assert!(large.can_hold(&small));
        assert!(!small.can_hold(&large));
    }

    #[test]
    fn assert_eq() {
        assert_eq!(1, 1);
    }
    #[test]
    fn assert_ne() {
        assert_ne!(2, 1);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn assert_msg() {
        assert!(true);
    }

    #[test]
    #[should_panic(expected = "div_zero")]
    fn div_zero() {
        panic!("div_zero");
    }

    #[test]
    fn result() -> Result<(), String> {
        let i = 3;
        if i == 3 {
            Ok(())
        } else {
            Err(String::from("error"))
        }
    }
}
