use crate::http_std::Http;

// `async` generates state machines: start,wait,done.
// `await` progress through the states by poll()-ing the Future.
fn main() {
    use std::{future::Future, task::Context, task::Poll};
    let fut = async_main_standard();
    let waker = dummy_waker::dummy_waker();
    let mut context = Context::from_waker(&waker);

    // State machine generated by `async` keyword has self-referential types, 
    // for this reason we Pin the future, so it does not Move in the memory before poll()-ing it.
    let mut pin = Box::pin(fut);

    loop {
        match pin.as_mut().poll(&mut context) {
            Poll::Pending => {}
            Poll::Ready(_) => {
                break;
            }
        }
    }
}


async fn async_main_standard() {
    let txt = Http::new("/400/helloworld").await;
    println!("{txt}")
}