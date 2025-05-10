#![feature(
    impl_trait_in_assoc_type,
    auto_traits,
    negative_impls,
    macro_metavar_expr,
    const_precise_live_drops
)]


mod socket_addrs;
mod manually_poll;
mod dirty;
mod vector;


pub use voxidian_protocol;

pub use uuid;
pub use rand;

pub use clap;

pub use tokio;

pub mod bevy {
    pub use bevy_app as app;
    pub use bevy_defer as defer;
    pub use bevy_diagnostic as diagnostic;
    pub use bevy_ecs as ecs;
    pub use bevy_time as time;
    pub mod prelude {
        pub use bevy_app::{ self, prelude::* };
        pub use bevy_defer::{ self,
            AsyncWorld,
            AsyncCommandsExtension,
            AsyncEntityCommandsExtension,
            Task
        };
        pub use bevy_ecs::{ self,
            prelude::*
        };
        pub use bevy_time::{ self, prelude::* };
    }
}



pub mod prelude {

    pub use crate::socket_addrs::SocketAddrs;
    pub use crate::manually_poll::ManuallyPoll;
    pub use crate::dirty::Dirty;
    pub use crate::vector::*;

    pub use core::array;
    pub use core::cell::LazyCell;
    pub use core::fmt::Debug;
    pub use core::marker::PhantomData;
    pub use core::mem;
    pub use core::net::{ AddrParseError, SocketAddr };
    pub use core::ops::{ Deref, DerefMut };
    pub use core::str::FromStr;
    pub use core::sync::atomic::{
        AtomicBool,
        Ordering as AtomicOrdering
    };
    pub use core::task::Poll;
    pub use core::time::Duration;
    pub use std::borrow::Cow;
    pub use std::collections::VecDeque;
    pub use std::io;
    pub use std::path::{ Path, PathBuf };
    pub use std::process;
    pub use std::sync::Arc;
    pub use std::time::Instant;

    pub use crate::voxidian_protocol as protocol;

    pub use crate::uuid::Uuid;
    pub use crate::rand;

    pub use crate::clap;
    pub use crate::clap::Parser;

    pub use crate::tokio;
    pub use crate::tokio::io::AsyncWriteExt;
    pub use crate::tokio::net::{
        TcpListener,
        TcpStream,
        tcp::{
            OwnedReadHalf,
            OwnedWriteHalf
        }
    };
    pub use crate::tokio::sync::{
        Mutex,
        MutexGuard,
        RwLock,
        RwLockReadGuard,
        RwLockWriteGuard
    };

    pub use crate::bevy;
    pub use crate::bevy::prelude::*;

    pub struct DefaultPlugins;
    impl Plugin for DefaultPlugins {
        fn build(&self, app : &mut App) {
            use core::time::Duration;
            app
                .add_plugins(bevy_app        ::ScheduleRunnerPlugin::run_loop(Duration::ZERO))
                .add_plugins(bevy_defer      ::AsyncPlugin::default_settings())
                .add_plugins(bevy_diagnostic ::DiagnosticsPlugin)
                .insert_resource(bevy_diagnostic::FrameCount::default())
                .add_plugins(bevy_time       ::TimePlugin);
        }
    }

    #[inline(always)]
    pub fn default<T : Default>() -> T { Default::default() }

    pub fn handle_err<T, E : Debug>(r : Result<T, E>) -> Result<T, ()> {
        match (r) {
            Ok(t) => Ok(t),
            Err(err) => {
                println!("FATAL ERROR {:?}", err); // TODO: Better error message.
                AsyncWorld.send_event(AppExit::error()).unwrap();
                Err(())
            },
        }
    }

}
