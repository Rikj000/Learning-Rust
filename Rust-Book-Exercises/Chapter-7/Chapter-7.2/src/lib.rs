mod front_of_house {
    mod hosting {
        fn add_to_wait_list() { println!("Hosting - Adding to wait list..."); }
        fn seat_at_table() { println!("Hosting - Seating at table..."); }
    }

    mod service {
        fn take_order() { println!("Service - Taking order..."); }
        fn serve_order() { println!("Service - Serving Order..."); }
        fn take_payment() { println!("Service - Taking payment..."); }
    }
}