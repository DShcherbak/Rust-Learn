enum OldIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
struct Ipv4Addr {
    // --snip--
}

#[allow(dead_code)]
struct Ipv6Addr {
    // --snip--
}

#[allow(dead_code)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

 impl Message {
    fn call(&self) -> String {
        match self {
            Message::Quit => String::from("Quit"),
            Message::Move { x, y } => format!("Move to ({x},{y})"),
            Message::Write(m) => format!("Message: {}", m),
            Message::ChangeColor(a, b, c) => format!("New color: ({a},{b},{c})"),
            
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _home = OldIpAddr::V4(127,0,0,1);
    let _loopback = OldIpAddr::V6(String::from("::1"));
    

    let m = Message::Write(String::from("hello"));
    assert_eq!(m.call(), "Message: hello");

    let m2 = Message::Move{x:3,y:2};
    assert_eq!(m2.call(), "Move to (3,2)");

    let some_number = Some(5);

    let absent_number: Option<i32> = None;

    if let Some(x) = some_number  {
        assert_eq!(x, 5);
    } else {
        assert_eq!(some_number, None);
    }

    if let Some(x) = absent_number  {
        assert_eq!(x, 5);
    } else {
        assert_eq!(absent_number, None);
    }

    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    assert_eq!(six, Some(6));

    assert_eq!(value_in_cents(Coin::Penny), 1);

    let dice_roll = 9;
    match dice_roll {
        3 => assert_eq!(dice_roll, 3),
        7 => assert_eq!(dice_roll, 7),
        other => {
            assert_ne!(other, 3);
            assert_ne!(other, 7);
        },
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        assert!(true);
        assert_eq!(max, 3);
    } else {
        assert!(false);
    }

    println!("All tests passed!");
}
