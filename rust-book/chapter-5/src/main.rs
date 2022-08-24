struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct UnitLike;

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
        self.width > other.width && self.height > other.height ||
            self.width > other.height && self.height > other.width
    }

    fn Square(a: u32) -> Rectangle{
        Rectangle { width: a, height: a }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    assert_eq!(user1.username, "someusername123");
 
    user1.email = String::from("anotheremail@example.com");
    assert_eq!(user1.email, "anotheremail@example.com");

    let user2 = build_user(user1.email.clone(), user1.username.clone());
    assert_eq!(user1.active, user2.active); // can't compare users yet

    let user1_copy = User {
        email: String::from("email@com"),
        ..user1
    };
    assert_eq!(user1.active, user1_copy.active);
    assert_eq!(user1.sign_in_count, user1_copy.sign_in_count);
  
    // Can't compare username, because it is a heap value, and it was moved 
    // when created user1_copy
    //  assert_eq!(user1.username, user1_copy.username);
    assert_ne!(user1.email, user1_copy.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    assert_eq!(black.0, origin.0);
    //but can't compare different types: assert_eq!(black, origin);

    let _always_equal = UnitLike;
    let _always_equal2 = UnitLike;
    //still need to derive Eq to compare
    //assert_eq!(always_equal, always_equal2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    assert_eq!(1500, rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle { width: 60, height: 45 };

    assert!(rect1.can_hold(&rect2));
    assert!(!rect1.can_hold(&rect3));

    let square = Rectangle::Square(25);
    assert_eq!(square.width, square.height);




    println!("All tests passed!");
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
