fn main() {
    let x = 5;
    // x = 6 would be an error

    //shadowing, old x = 5 is dropped
    let x = x + 1;

    {
        //in-scope shadowing, dropped at the end of scope
        let x = x * 2;
        assert_eq!(x, 12);
        //in-scope shadow dropped, x is back to 6
    }
    assert_eq!(x, 6);

    let t = true;
    let f: bool = false; // with explicit type annotation

    assert!(t == !f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    assert!(c < z && z < heart_eyed_cat);

    let spaces = "   ";
    //shadowing changes type
    let spaces = spaces.len();
    assert_eq!(spaces, 3);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, z) = tup;
    assert_eq!(y, 6.4);
    assert_eq!(z, 1);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a = [3; 5]; //[3,3,3,3,3]
    another_function(a[0]);

    let y = {
        let x = 3;
        x + 1
    };

    assert_eq!(y, 4);

    let six = plus_one(five());
    assert_eq!(six, 6);

    let number = 3;

    if number < 5 {
        assert!(number < 5);
    } else {
        assert!(number >= 5);
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    assert_eq!(number, 5);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    nested_loops_labels();
    while_example();
    foreach();

    println!("You understood everything correctly!");

}

fn another_function(x: i32) {
    assert!(x == 3);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn nested_loops_labels(){
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    assert_eq!(count, 2);
}

fn while_example(){
    let mut number = 3;

    while number != 0 {

        number -= 1;
    }

    assert_eq!(number, 0);
}

fn foreach(){
    let mut sum = 0;
    let a = [10, 20, 30, 40, 50];

    for element in a {
        for number in (1..4).rev() {
            sum += element + number;
        }
    }

    assert_eq!(sum, 480); //150 * 3 + (3+2+1) * 5 =
                          //= 450 + 30 = 480
                          //because (1..4) means [1;4) or [1;3]
                          //and (1..=4) means [1;4]
}