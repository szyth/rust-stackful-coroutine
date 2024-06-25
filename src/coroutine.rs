use crate::{future::{self, Future, PollState}, http::Http};


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
// At the Wait state, it can yield back to the Caller, Another Coroutine or a Scheduler

// a Coroutine will be a Non-leaf Future that will manage multiple leaf futures and resolve them (?)
// Coroutine will create a StateMachine for itself that will drive all its leaf futures to completion so that Task can be completed.
// State machines are compiler-generated in a real world.
pub struct Coroutine {
    state: StateMachine,
}

impl Coroutine {
   pub fn new() -> Self {
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