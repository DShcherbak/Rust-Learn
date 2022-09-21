pub mod hosting {
    //same for this, needs to be pub
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();
}

fn main(){

}