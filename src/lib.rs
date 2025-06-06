#![feature(
    // Language
    auto_traits,
    decl_macro,
    impl_trait_in_assoc_type,
    macro_metavar_expr,
    negative_impls
)]


mod socket_addrs;
mod manually_poll;
mod dirty;
mod ordered;
mod increment;
mod vector;
mod variadic;

pub fn handle_err<T, E : core::fmt::Display>(r : Result<T, E>) -> Result<T, E> {
    match (r) {
        Ok(t) => Ok(t),
        Err(err) => {
            flywheelmc_logging::fatal!("{}", err);
            bevy::defer::AsyncWorld.send_event(bevy::app::AppExit::error()).unwrap();
            Err(err)
        },
    }
}


pub use flywheelmc_logging;
pub use voxidian_protocol;

pub use uuid;
pub use rand;
pub use disqualified;
pub use chrono;
pub use bimap;

pub use clap;

pub use smol;
pub use futures;

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
            AsyncAccess,
            Task
        };
        pub use bevy_ecs::{ self, prelude::{
            Commands,
            Component,
            Entity,
            Event,
            EventReader,
            EventWriter,
            Query,
            Res,
            Resource,
            With
        } };
        pub use bevy_time::{ self, prelude::* };
    }
}



pub mod prelude {

    pub use crate::socket_addrs::SocketAddrs;
    pub use crate::manually_poll::ManuallyPoll;
    pub use crate::dirty::Dirty;
    pub use crate::ordered::Ordered;
    pub use crate::increment::Increment;
    pub use crate::vector::*;
    pub use crate::variadic::variadic;
    pub use crate::handle_err;

    pub use crate::flywheelmc_logging::{
        fatal, fatal_once,
        error, error_once,
        warn,  warn_once,
        pass,  pass_once,
        info,  info_once,
        debug, debug_once,
        trace, trace_once,
        once,
        LevelFilter, LogTarget, SingleLogTarget,
        ENABLE_COLOUR, GLOBAL_FILTER, LOG_TARGETS
    };
    pub use crate::voxidian_protocol as protocol;

    pub type PlotId = u64;

    pub use core::array;
    pub use core::cell::LazyCell;
    pub use core::fmt::{ self, Debug, Display };
    pub use core::iter;
    pub use core::marker::PhantomData;
    pub use core::mem::{
        self,
        ManuallyDrop,
        MaybeUninit
    };
    pub use core::net::{ AddrParseError, SocketAddr };
    pub use core::num::NonZeroU8;
    pub use core::ops::{ Deref, DerefMut };
    pub use core::pin::Pin;
    pub use core::slice;
    pub use core::str::{ self, FromStr };
    pub use core::sync::atomic::{
        AtomicBool,
        AtomicU64,
        AtomicUsize,
        Ordering as AtomicOrdering
    };
    pub use core::task::Poll;
    pub use core::time::Duration;
    pub use std::borrow::Cow;
    pub use std::collections::{
        BTreeMap,
        BTreeSet,
        VecDeque
    };
    pub use std::collections::HashMap;
    pub use std::fs;
    pub use std::io;
    pub use std::path::{ Path, PathBuf };
    pub use std::process;
    pub use std::sync::{
        Arc,
        Mutex as SMutex,
        RwLock as SRwLock,
        LazyLock
    };
    pub use std::time::{ Instant, SystemTime };

    pub use crate::uuid::Uuid;
    pub use crate::rand::{ random, random_range };
    pub use crate::disqualified::ShortName;
    pub use crate::chrono::Utc;
    pub use crate::bimap::BiBTreeMap;

    pub use crate::clap;
    pub use crate::clap::Parser;
    pub use crate::clap::ColorChoice as ColourChoice;

    pub trait CLIParse : Sized {
        fn parse_check_colour() -> Self;
    }
    impl<P : clap::Parser> CLIParse for P {
        fn parse_check_colour() -> Self {
            let mut cmd = <Self as clap::CommandFactory>::command()
                .color(if (*ENABLE_COLOUR) { ColourChoice::Always } else { ColourChoice::Never });
            let mut matches = cmd.get_matches_mut();
            let res = <Self as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
                .map_err(|err| err.format(&mut cmd));
            match (res) {
                Ok(p) => p,
                Err(err) => { err.exit(); }
            }
        }
    }

    pub use crate::smol;
    pub use crate::smol::io::{
        AsyncWriteExt,
        AsyncReadExt
    };
    pub use crate::smol::net::{
        TcpListener,
        TcpStream
    };
    pub use crate::smol::lock::{
        Mutex,
        MutexGuard,
        RwLock,
        RwLockReadGuard,
        RwLockWriteGuard
    };
    pub use crate::smol::channel;
    pub mod task {
        #[inline(always)]
        pub async fn sleep(dur : core::time::Duration) -> () {
            crate::bevy::defer::AsyncWorld.sleep(dur).await;
        }
        #[inline(always)]
        pub async fn yield_now() -> () {
            crate::bevy::defer::AsyncWorld.yield_now().await;
        }
        pub async fn poll_and_yield<T, Fut>(fut : Fut) -> T
        where
            Fut : Future<Output = T>
        {
            let     waker = core::task::Waker::noop();
            let mut ctx   = core::task::Context::from_waker(waker);
            let mut fut   = core::pin::pin!(fut);
            loop {
                match (fut.as_mut().poll(&mut ctx)) {
                    core::task::Poll::Pending => { },
                    core::task::Poll::Ready(out) => { return out; }
                }
                yield_now().await;
            }
        }
        pub async fn timeout<T, Fut>(dur : core::time::Duration, fut : Fut) -> Option<T>
        where
            Fut : Future<Output = T>
        {
            let     expires = std::time::Instant::now() + dur;
            let     waker   = core::task::Waker::noop();
            let mut ctx     = core::task::Context::from_waker(waker);
            let mut fut     = core::pin::pin!(fut);
            loop {
                match (fut.as_mut().poll(&mut ctx)) {
                    core::task::Poll::Pending => { },
                    core::task::Poll::Ready(out) => { return Some(out); }
                }
                if (std::time::Instant::now() >= expires) { return None; }
                yield_now().await;
            }
        }
        pub use crate::futures::executor::block_on;
    }

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

}
