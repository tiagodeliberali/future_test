use futures::future::{BoxFuture, FutureExt};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

pub struct Join {
    a: Arc<Mutex<Option<BoxFuture<'static, ()>>>>,
    b: Arc<Mutex<Option<BoxFuture<'static, ()>>>>,
}

impl Future for Join {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut optional_a = self.a.lock().unwrap();
        if let Some(mut a) = optional_a.take() {
            if let Poll::Pending = a.as_mut().poll(cx) {
                *optional_a = Some(a);
            }
        }

        // Attempt to complete future `b`.
        let mut optional_b = self.b.lock().unwrap();
        if let Some(mut b) = optional_b.take() {
            if let Poll::Pending = b.as_mut().poll(cx) {
                *optional_b = Some(b);
            }
        }

        if optional_a.is_none() && optional_b.is_none() {
            // Both futures have completed-- we can return successfully
            Poll::Ready(())
        } else {
            // One or both futures returned `Poll::Pending` and still have
            // work to do. They will call `wake()` when progress can be made.
            Poll::Pending
        }
    }
}

impl Join {
    pub fn new(
        future1: impl Future<Output = ()> + 'static + Send,
        future2: impl Future<Output = ()> + 'static + Send,
    ) -> Join {
        Join {
            a: Arc::new(Mutex::new(Some(future1.boxed()))),
            b: Arc::new(Mutex::new(Some(future2.boxed()))),
        }
    }
}

pub struct AndThenFut {
    first: Arc<Mutex<Option<BoxFuture<'static, ()>>>>,
    second: Arc<Mutex<BoxFuture<'static, ()>>>,
}

impl Future for AndThenFut {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut optional_first = self.first.lock().unwrap();
        if let Some(mut first) = optional_first.take() {
            if let Poll::Pending = first.as_mut().poll(cx) {
                *optional_first = Some(first);
                return Poll::Pending;
            }
        }

        // Now that the first future is done, attempt to complete the second.
        let mut optional_second = self.second.lock().unwrap();
        optional_second.as_mut().poll(cx)
    }
}

impl AndThenFut {
    pub fn new(
        future1: impl Future<Output = ()> + 'static + Send,
        future2: impl Future<Output = ()> + 'static + Send,
    ) -> AndThenFut {
        AndThenFut {
            first: Arc::new(Mutex::new(Some(future1.boxed()))),
            second: Arc::new(Mutex::new(future2.boxed())),
        }
    }
}
