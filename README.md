- I have created a Stackless Coroutine, a pausable/resumable task using a Custom Future trait with a hand-written State Machine that progresses through our Types to completion that implement this Custom Future trait.
- We poll each Future type to completion until all the Poll Pending states return Poll Ready state.

- For event notification I have used tokio's MIO, and no other dependencies are used in the codebase.

- This way we can run tasks Concurrently utilizing full CPU resources.