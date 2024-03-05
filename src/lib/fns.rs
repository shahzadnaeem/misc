pub fn fn_stuff() {
    let ref_s = "Hey!".to_string();
    let mut mut_s = "Hey!".to_string();
    let move_s = "Hey!".to_string();

    let hey = |name: &str| format!("{} {}", &ref_s, name);
    let mut hey_mut = |name: &str| {
        mut_s += "!";
        format!("{} {}", &mut_s, name)
    };
    let hey_once = |name: &str| {
        let do_hey_move = |hey: String, name: &str| format!("{} {}", hey, name);

        do_hey_move(move_s, name)
    };

    println!("hey(): {}", hey("Shaz"));
    println!("hey_mut(): {}", hey_mut("Shaz"));
    println!("hey_mut(): {}", hey_mut("Shaz"));
    println!("hey_move: {}", hey_once("Shaz"));
    // println!("hey_move: {}", hey_once("Shaz"));  // NOTE: Can't call this a second time
}
