#![cfg_attr(feature = "cargo-clippy", allow(module_inception))]

#[macro_use]
extern crate lazy_static;

use failure;

pub use crate::packet::constants;
pub use netlink_packet as packet;
use netlink_proto;
pub use netlink_proto::{Connection, Protocol};
pub use netlink_sys;

mod handle;
pub use crate::handle::*;

mod errors;
pub use crate::errors::*;

use crate::packet::NetlinkMessage;
use futures::sync::mpsc::UnboundedReceiver;

use std::io;

pub fn new_connection() -> io::Result<(Connection, Handle, UnboundedReceiver<NetlinkMessage>)> {
    let (conn, handle, messages) = netlink_proto::new_connection(Protocol::Audit)?;
    Ok((conn, Handle::new(handle), messages))
}
