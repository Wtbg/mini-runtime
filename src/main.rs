mod runtime;
use std::{pin::Pin, task::{Poll, Context}};

use futures::Future;
pub use runtime::{block_on, spawn};

struct FutureOne;
impl Future for FutureOne {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("FutureOne polled");
        Poll::Ready(1)
    }
}

async fn main_async() -> u32 {
    let one = FutureOne.await;
    println!("FutureOne1 returned: {}", one);
    let two = FutureOne.await;
    println!("FutureOne2 returned: {}", two);
    one + two
}

fn main() {
    let future = main_async();
    let output = block_on(future);
    println!("main_async returned: {}", output);
    println!("{}", output);
}


