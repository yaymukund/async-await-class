#![feature(async_await, await_macro, futures_api, pin)]

use std::future::{Future, FutureObj};
use std::mem::PinMut;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{sync_channel, SyncSender, SendError, Receiver};
use std::task::{
    self,
    Executor,
    local_waker_from_nonlocal,
    Poll,
    SpawnErrorKind,
    SpawnObjError,
    Wake,
};

struct Exec {
    task_sender: SyncSender<Arc<Task>>,
    task_receiver: Receiver<Arc<Task>>,
}

impl<'a> Executor for &'a Exec {
    fn spawn_obj(&mut self, future: FutureObj<'static, ()>)
        -> Result<(), SpawnObjError>
    {
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).map_err(|SendError(task)| {
            SpawnObjError {
                kind: SpawnErrorKind::shutdown(),
                future: task.future.lock().unwrap().take().unwrap(),
            }
        })
    }
}

struct Task {
    future: Mutex<Option<FutureObj<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

fn main() {
}