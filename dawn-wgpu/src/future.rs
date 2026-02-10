use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

pub(crate) struct CallbackFuture<T> {
    pub(crate) shared: Arc<Mutex<CallbackShared<T>>>,
}

pub(crate) struct CallbackShared<T> {
    pub(crate) result: Option<T>,
    pub(crate) waker: Option<Waker>,
}

impl<T> CallbackFuture<T> {
    pub(crate) fn new() -> (Self, Arc<Mutex<CallbackShared<T>>>) {
        let shared = Arc::new(Mutex::new(CallbackShared {
            result: None,
            waker: None,
        }));
        (
            Self {
                shared: shared.clone(),
            },
            shared,
        )
    }
}

impl<T> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared = self.shared.lock().expect("wgpu-compat future lock");
        if let Some(result) = shared.result.take() {
            return Poll::Ready(result);
        }
        shared.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

pub(crate) fn complete_shared<T>(shared: &Arc<Mutex<CallbackShared<T>>>, value: T) {
    let mut shared = shared.lock().expect("wgpu-compat future lock");
    shared.result = Some(value);
    if let Some(waker) = shared.waker.take() {
        waker.wake();
    }
}
