#![allow(dead_code)]

use std::{
    sync::{Arc, Mutex},
    task::Waker,
};

fn spawn_blocking<T, F>(closure: F) -> SpawnBlocking<T>
where
    T: Send + 'static,
    F: FnOnce() -> T,
    F: Send + 'static,
{
    let inner = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
    }));

    std::thread::spawn({
        let inner = inner.clone();
        move || {
            let value = closure();

            let maybe_waker = {
                let mut guard = inner.lock().unwrap();
                guard.value = Some(value);
                guard.waker.take()
            };

            if let Some(waker) = maybe_waker {
                waker.wake();
            }
        }
    });

    SpawnBlocking(inner)
}

struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

use std::future::Future;

impl<T: Send> Future for SpawnBlocking<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut guard = self.0.lock().unwrap();
        if let Some(value) = guard.value.take() {
            return std::task::Poll::Ready(value);
        }

        guard.waker = Some(cx.waker().clone());
        std::task::Poll::Pending
    }
}

struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

use crossbeam::sync::Parker; // Cargo.toml: crossbeam = "0.8"
use futures_lite::pin; // Cargo.toml: futures-lite = "1.11"
use std::task::{Context, Poll};
use waker_fn::waker_fn; // Cargo.toml: waker-fn = "1.1"

fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);

    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}

fn main() {
    // I gave up temporarily on Async Rust...the basics are good, but anythign past the actual implementations hurts my head at the moment
    // I am also terribly looking for a good paying good at the moment, so I am not very motivated to spend days on this now, would revert back to this
}
