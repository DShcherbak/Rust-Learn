fn main() {
    {  // _s is not valid here, it’s not yet declared
        let _s :&str = "hello";   // s is valid from this point forward
        // do stuff with _s
    }  
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    assert_eq!(s, String::from("Hello, world!"));

    let x = 5;
    let y = x;
    assert_eq!(x,y);

    let s1 = String::from("Hello");
    let s2 = s1;
    let s3 = s2.clone();
    assert_eq!(s2, "Hello");
    assert_eq!(s2, s3);
    //s1 is no longer valid here, but s2 is

    takes_ownership(s2);
    //now s2 is not valid
    takes_ownership(s3.clone());
    assert_eq!(s3, "Hello");

    let mut s4 = String::from("Hello");
    let len = calculate_length(&s4);
    assert_eq!(len, 5);

    //This won't work:
    //let r1 = &mut s;
    //let r2 = &mut s;

    //But this does:
    let r1 = &mut s4;
    change(r1);
    //now, since r1 is not used anymore, we can create r2
    let r2 = &mut s4;
    change(r2);

    //if I try to use r1 here, everything breaks
    //Rust good, rust smart!!

    assert_eq!(s4, "Hello, world, world");

    let slice_hello  = &s4[..5];
    let slice_world1 = &s4[7..12];
    let slice_world2 = &s4[14..];
    let slice_full   = &s4[..];
    assert_eq!(slice_world1, slice_world2); 
    assert_eq!(slice_full, s4); 
    assert_eq!(slice_hello, no_dangle());

    let s = String::from("hello world");
    let word = first_word(&s);
    //Can't do this:  
    //s.clear();

    //Because s.clear() mutates, while first_word received an immutable borrow on s
    assert_eq!(word.len(),5);

    //String is str
    assert_eq!(s, s[..]);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; //slice of i32 array
    assert_eq!(slice, &[2, 3]); 

    


    println!("Everything works correctly!");
}

fn takes_ownership(mut s: String) {
    s.clear();
    assert_eq!(s, "");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//Broken fn, reference will exist but the string 
//will be deaclloc since out of scope

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}

fn first_word(s : &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..] 
}

