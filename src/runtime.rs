use std::{
    future::Future,
    task::{ Poll, Wake, Waker },
    sync::{ Condvar, Mutex, Arc },
    cell::RefCell,
};
use futures::future::BoxFuture;
use std::collections::VecDeque;
use lazy_static::lazy_static;
lazy_static! {
    static ref RUNNABLE: Mutex<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::with_capacity(100));
}
pub fn block_on<F: Future>(future: F) -> F::Output
    where F: Future + 'static + Send, F::Output: Default
{
    let mut future = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());
    let mut context = std::task::Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        let mut runnable = RUNNABLE.lock().unwrap();
        if let Some(task) = runnable.pop_front() {
            let mut future = task.future.borrow_mut();
            let waker = Waker::from(task.signal.clone());
            let mut context = std::task::Context::from_waker(&waker);
            if let Poll::Ready(()) = future.as_mut().poll(&mut context) {
                continue;
            }
            runnable.push_back(task.clone());
        }
        signal.wait();
    }
}

struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}
enum State {
    Empty,
    Waiting,
    Notified,
}

impl Signal {
    fn new() -> Self {
        Signal {
            state: Mutex::new(State::Empty),
            cond: Condvar::new(),
        }
    }
    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => {
                *state = State::Empty;
            }
            State::Waiting => {
                panic!("unexpected state");
            }
            State::Empty => {
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }
    fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Empty => {
                *state = State::Notified;
            }
            State::Waiting => {
                *state = State::Empty;
                self.cond.notify_one();
            }
            State::Notified => {}
        }
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}
impl Wake for Task {
    fn wake(self: Arc<Self>) {
        let mut runnable = RUNNABLE.lock().unwrap();
        runnable.push_back(self.clone());
        self.signal.notify();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        let mut runnable = RUNNABLE.lock().unwrap();
        runnable.push_back(self.clone());
        self.signal.notify();
    }
}
impl Drop for Task {
    fn drop(&mut self) {
        self.signal.notify();
    }
} 






pub fn spawn<F>(future: F) where F: Future<Output = ()> + 'static + Send {
    let future = Box::pin(future);
    let task = Arc::new(Task {
        future: RefCell::new(future),
        signal: Arc::new(Signal::new()),
    });
    let mut runnable = RUNNABLE.lock().unwrap();
    runnable.push_back(task);
}
