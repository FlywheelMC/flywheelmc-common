use core::task::{ Poll, Waker, Context };
use core::pin::Pin;


pub struct ManuallyPoll<'l, T> {
    ctx : Context<'static>,
    fut : Pin<Box<dyn Future<Output = T> + 'l>>
}

impl<'l, T> ManuallyPoll<'l, T> {

    pub fn new<F>(fut : F) -> Self
    where
        F : Future<Output = T> +'l
    {
        Self {
            ctx : Context::from_waker(Waker::noop()),
            fut : Box::pin(fut)
        }
    }

    pub fn poll(&mut self) -> Poll<T> {
        self.fut.as_mut().poll(&mut self.ctx)
    }

}

unsafe impl<'l, T> Send for ManuallyPoll<'l, T> { }
unsafe impl<'l, T> Sync for ManuallyPoll<'l, T> { }
