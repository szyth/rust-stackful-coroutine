// A Lazy Future Coroutine implementation
// meaning the future needs to be poll()-ed to kick off their operation
// unlike eager futures in Javascript, where they start off as soon as they are created without needing the first poll().
// Note: This code is NOT an example of Concurrency. Instead it shows how futures work with state machines in stackless coroutines.
use coroutine::Coroutine;
use future::{Future, PollState};

mod coroutine;
mod future;
mod http;
mod http_std;
mod main_std;

// Equivalent Code using standard async/await and standard Futures: check `main_std.rs`
// prefixing a function with standard `async` will auto-generate a statemachine for that Task AKA Coroutine AKA Function
// unlike below where we manually generate the StateMachine
fn async_main_custom() -> impl Future<Output = ()> {
    // check `coroutine.rs` for definitions
    Coroutine::new()
}

fn main() {
    let mut fut = async_main_custom();

    // Runtime Executor will replace this loop. (?)
    loop {
        match fut.poll() {
            PollState::NotReady => {
                // future is NotReady so Control is Yielded back to Us by the statemachine.
                // so I can schedule other tasks this time
            }
            PollState::Ready(_) => {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // fut is in Resolved state at this step: so calling poll() again will panic!()
    // fut.poll();
}
