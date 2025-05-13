use core::fmt;
use core::net::SocketAddr;
use core::ops::Deref;
use core::str::FromStr;
use std::io;
use std::net::ToSocketAddrs;
use tokio::net;
use futures::{ executor, future };


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

impl fmt::Display for SocketAddrs {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, addr) in self.0.iter().enumerate() {
            if (i > 0) { write!(f, ",")?; }
            write!(f, "{}", addr)?;
        }
        Ok(())
    }
}

impl Deref for SocketAddrs {
    type Target = Vec<SocketAddr>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl FromStr for SocketAddrs {
    type Err = io::Error;
    fn from_str(s : &str) -> io::Result<Self> {
        executor::block_on(async {
            Ok(SocketAddrs(future::join_all(
                s.split(",")
                    .map(|r| net::lookup_host(r))
            ).await
                .into_iter()
                .collect::<io::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
            ))
        })
    }
}
