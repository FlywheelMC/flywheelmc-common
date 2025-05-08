use core::net::{
    AddrParseError,
    SocketAddr
};
use core::ops::Deref;
use core::str::FromStr;
use std::io;
use std::net::ToSocketAddrs;


#[derive(Debug, Clone)]
pub struct SocketAddrs(Vec<SocketAddr>);

impl SocketAddrs {

    pub const EMPTY : Self = Self(Vec::new());

    pub fn into_inner(self) -> Vec<SocketAddr> { self.0 }

}

impl ToSocketAddrs for SocketAddrs {
    type Iter = impl Iterator<Item = SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        Ok(self.0.clone().into_iter())
    }
}

impl Deref for SocketAddrs {
    type Target = Vec<SocketAddr>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl FromStr for SocketAddrs {
    type Err = AddrParseError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        s.split(",")
            .map(|r| SocketAddr::from_str(r))
            .collect::<Result<Vec<_>, _>>()
            .map(SocketAddrs)
    }
}