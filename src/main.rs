// A Lazy Future Coroutine implementation
// meaning the future needs to be poll()-ed to kick off their operation
// unlike eager futures in Javascript, where they start off as soon as they are created without needing the first poll().
// Note: This code is NOT an example of Concurrency. Instead it shows how futures work with state machines in stackless coroutines.
use future::{Future, PollState};
use http::Http;

mod future;
mod http_std;
mod http;
mod main_std;

// A state machine stores the state of a Pausable/resumable Function.
// It is a data structure (and not an OS stack like in Green Threads/Fibres/Stacklful coroutine).
// These are auto-generated when we write `async`
enum StateMachine {
    Start,
    Wait(Box<dyn Future<Output = String>>), // save State of the future in wait state
    Resolved,
}

// Coroutine (stackless) is a A stoppable/resumable Function or Task 
// Coroutines are compiled into StateMachine
// Coroutine is a Non Leaf future (?)
// At the Wait state, it can yield back to the Caller, Another Coroutine or a Scheduler
struct Coroutine {
    state: StateMachine,
}

impl Coroutine {
    fn new() -> Self {
        Self {
            state: StateMachine::Start,
        }
    }
}

impl Future for Coroutine {
    type Output = ();
    fn poll(&mut self) -> future::PollState<Self::Output> {
        loop {
            match self.state {
                StateMachine::Start => {
                    let fut = Box::new(Http::new("/400/helloworld"));
                    self.state = StateMachine::Wait(fut);
                }
                StateMachine::Wait(ref mut fut) => match fut.poll() {
                    PollState::Ready(data) => {
                        self.state = StateMachine::Resolved;
                        print!("Im fully resolved Future: \n{data}\n");
                        break PollState::Ready(());
                    }
                    PollState::NotReady => {
                        break PollState::NotReady;
                    }
                },
                StateMachine::Resolved => {
                    panic!("Future already resolved")
                }
            }
        }
    }
}


// Equivalent Code using standard async/await and standard Futures: check `main_std.rs`
// prefixing a function with standard `async` will auto-generate a statemachine for that Task AKA Coroutine AKA Function
// unlike below where we manually generate the StateMachine
fn async_main_custom() -> impl Future<Output = ()> {
    Coroutine::new()
}



fn main() {
    use future::{Future, PollState};
    // a Coroutine will be a Non-leaf Future that will manage multiple leaf futures and resolve them.
    // Coroutine will create a StateMachine for itself that will drive all its leaf futures to completion so that Task can be completed.
    // State machines are compiler-generated in a real world.
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

