#[path = "lib/slices.rs"]
mod slices;

pub use slices::new_vec;

#[path = "lib/packets.rs"]
mod packets;

#[path = "lib/hex_dump.rs"]
pub mod hex_dump;

#[path = "lib/fns.rs"]
pub mod fns;

pub use fns::fn_stuff;

#[path = "lib/tcp.rs"]
pub mod tcp;

pub use tcp::one_shot_tcp_server;
