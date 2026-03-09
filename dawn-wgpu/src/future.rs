use futures_util::task::AtomicWaker;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub(crate) struct CallbackFuture<T> {
    pub(crate) receiver: std::sync::mpsc::Receiver<T>,
    pub(crate) waker: Arc<AtomicWaker>,
}

pub(crate) struct CallbackCompletion<T> {
    sender: std::sync::mpsc::Sender<T>,
    waker: Arc<AtomicWaker>,
}

impl<T> CallbackFuture<T> {
    pub(crate) fn new() -> (Self, CallbackCompletion<T>) {
        let (sender, receiver) = std::sync::mpsc::channel();
        let waker = Arc::new(AtomicWaker::new());
        (
            Self {
                receiver,
                waker: waker.clone(),
            },
            CallbackCompletion { sender, waker },
        )
    }
}

impl<T> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.waker.register(cx.waker());
        match self.receiver.try_recv() {
            Ok(result) => Poll::Ready(result),
            Err(std::sync::mpsc::TryRecvError::Empty) => Poll::Pending,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                panic!("dawn-wgpu callback future channel disconnected")
            }
        }
    }
}

pub(crate) fn complete_shared<T>(shared: &CallbackCompletion<T>, value: T) {
    let _ = shared.sender.send(value);
    shared.waker.wake();
}
