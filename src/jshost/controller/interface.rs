use std::{
    collections::HashMap,
    env,
    process::{self, Command, Stdio},
    sync::{mpsc, Arc, Mutex, MutexGuard},
};

use ipc_channel::platform::{OsIpcOneShotServer, OsIpcReceiver, OsIpcSender};
use serde::{Deserialize, Serialize};
use zbus::dbus_interface;

use crate::backend::util::id::OBGId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    seq: u64,
    op: Op,
}

impl Message {
    pub fn init() -> Self {
        Message {
            seq: 0,
            op: Op::Init,
        }
    }
    pub fn op(&self) -> &Op {
        &self.op
    }
    pub fn next(&self, op: Op) -> Self {
        Message {
            seq: self.seq + 1,
            op,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Op {
    Init,
    InitComplete,
    ExecuteScript(String),
    ExecuteComplete(String),
}
pub struct JsInterface {
    pub workers: HashMap<OBGId, Worker>,
}

#[dbus_interface(name = "games.oogabooga.JsHost.JsInterface")]
impl JsInterface {
    fn create_game(&mut self, id: u64) {
        let worker = Worker::spawn();
        let msg = worker.init();
        let (tx, rx) = (worker.tx().unwrap(), worker.rx().unwrap());

        tx.send(&bincode::serialize(&msg).unwrap()[..], vec![], vec![])
            .unwrap();

        loop {
            match rx.recv() {
                Ok(data) => {
                    if let Ok(msg) = bincode::deserialize::<Message>(&data.0) {
                        match msg.op() {
                            Op::InitComplete => {
                                worker.next(Op::InitComplete);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }

        self.workers.insert(id.into(), worker.clone());
    }
    fn list_games(&self) {
        self.workers.keys().for_each(|id| {
            println!(
                "{}: {}",
                id,
                self.workers.get(id).unwrap().proc.lock().unwrap().id()
            );
        });
    }
}

#[derive(Clone)]
pub struct Worker {
    pub proc: Arc<Mutex<process::Child>>,
    tx: Arc<Mutex<OsIpcSender>>,
    rx: Arc<Mutex<OsIpcReceiver>>,
    message: Arc<Mutex<Option<Message>>>,
}

impl Worker {
    pub fn spawn() -> Self {
        let (server, name) = OsIpcOneShotServer::new().unwrap();
        let proc = Arc::new(Mutex::new(
            Command::new(env::current_exe().unwrap())
                .arg(format!("--worker={}", name))
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::null())
                .spawn()
                .expect("failed to execute server process"),
        ));

        let (tx, rx) = {
            let (rx, _, mut received_channels, _) = server.accept().unwrap();
            let tx = received_channels[0].to_sender();
            (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
        };

        let message = Arc::new(Mutex::new(None));

        Self {
            proc,
            tx,
            rx,
            message,
        }
    }

    pub fn tx(&self) -> Option<OsIpcSender> {
        if self.message.lock().unwrap().is_some() {
            return Some(self.tx.lock().unwrap().clone());
        }
        None
    }

    pub fn rx(&self) -> Option<MutexGuard<'_, OsIpcReceiver>> {
        if self.message.lock().unwrap().is_some() {
            return Some(self.rx.lock().unwrap());
        }
        None
    }

    pub fn next(&self, op: Op) -> Message {
        let mut message = self.message.lock().unwrap();
        let next = message.clone().unwrap().next(op);
        *message = Some(next.clone());
        next
    }

    pub fn init(&self) -> Message {
        let mut message = self.message.lock().unwrap();
        let next = Message::init();
        *message = Some(next.clone());
        next
    }
}
