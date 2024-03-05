use anyhow::Context;
#[allow(dead_code)]
use misc::hex_dump::{hex_dump_via_cmd, hex_dump_via_cmd_anyh};
pub use misc::{fn_stuff, new_vec, one_shot_tcp_server};

fn new_vec_stuff() {
    // Create
    let mut v: Vec<u32> = new_vec(10);

    println!("v = {:?}", v);

    // Modify in place
    v.iter_mut().enumerate().for_each(|(i, e)| *e = i as u32);

    println!("modified v = {:?}", v);

    // Map to new
    let mapped: Vec<_> = v.iter().map(|e| 10 - e).collect();

    println!("mapped = {:?}", mapped);

    // Extract and map
    let taken: Vec<_> = v.into_iter().map(|e| (100 - e) as u8).collect();

    println!("taken = {:?}", taken);

    // v is no longer available ...
}

fn more_vec_stuff() {
    let s = String::from("This is a long-ish string with some text");

    let s1 = &s[0..10];

    println!("s1 = {}", s1);

    let s2: Vec<_> = s1.chars().collect();

    println!("s2 = {:?}", s2);
}

fn copy_slice(src: &[u8], dest: &mut [u8], start: usize) -> bool {
    println!("copy_slice: src: {:?}, dest: {:?}", src, dest);

    if src.len() + start <= dest.len() {
        for i in 0..src.len() {
            dest[i + start] = src[i];
        }

        println!("copied: dest: {:?}", dest);
    } else {
        println!("Copy will FAIL :(");
        return false;
    }

    true
}

fn reverse(buf: &mut [u8]) {
    let sz = buf.len();

    for i in 0..(sz / 2) {
        (buf[i], buf[sz - i - 1]) = (buf[sz - i - 1], buf[i]);
    }
}

fn slice_stuff() {
    let src = [5, 4, 3];
    let mut dest = [0; 10];

    copy_slice(&src, &mut dest, 4);
    assert_eq!([0, 0, 0, 0, 5, 4, 3, 0, 0, 0], dest);

    copy_slice(&src, &mut dest, 7);
    assert_eq!([0, 0, 0, 0, 5, 4, 3, 5, 4, 3], dest);

    let mut buf: [u8; 4] = [0u8; 4];
    for i in 0..buf.len() {
        buf[i] = i as u8;
    }

    println!("buf: {:?}", buf);
    reverse(&mut buf);
    println!("reversed buf: {:?}", buf);
}

fn main() -> anyhow::Result<()> {
    if false {
        new_vec_stuff();

        more_vec_stuff();

        slice_stuff();

        fn_stuff();

        one_shot_tcp_server();
    }

    let s1 = "Hello";
    let s2 = "Hell\0\tllo";

    let output = format!("s1: {}, s2: {}", s1, s2);
    let cmd = "od";
    let args = ["-A", "x", "-t", "x1z", "-v"];
    // let args = ["-A"]; // NOTE: to trigger errors

    // Manually managed error handling
    let mut ok = true;
    let res = hex_dump_via_cmd(&output, cmd, &args).unwrap_or_else(|e| {
        ok = false;
        format!(
            "Error: Command failed: {cmd} {}\n\t{} - from: {:?}\n",
            args.join(" "),
            e,
            e
        )
    });
    println!("\nStatus: {}\n{}", if ok { "GOOD" } else { "ERROR" }, res);

    // Rusty + anyhow error handling
    let res = hex_dump_via_cmd_anyh(&output, cmd, &args)
        .with_context(|| format!("Command failed: {} {}", cmd, args.join(" ")))?;

    print!("\nStatus: GOOD\n{}", res);

    Ok(())
}
