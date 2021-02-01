//! # WebAssembly / JavaScript support
//!
use crate::bpmn;
use crate::bpmn::schema::BaseElementType;
use crate::model::Model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::oneshot;
use wasm_bindgen::prelude::*;
use wasm_rs_shared_channel::spsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    id: u32,
    variant: Variant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Variant {
    Info,
    CreateModel { xml: String },
    ListProcesses { model_id: u32 },
    StartProcess { model_id: u32, process_id: u32 },
    SubscribeProcessLog { model_id: u32, process_id: u32 },
}

#[wasm_bindgen]
pub struct Channel {
    sender: Option<spsc::Sender<Request>>,
    receiver: Option<spsc::Receiver<Request>>,
}

#[wasm_bindgen]
impl Channel {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Channel {
        let (sender, receiver) = spsc::channel::<Request>(1024 * 1024).split();
        Channel {
            sender: Some(sender),
            receiver: Some(receiver),
        }
    }
    pub fn from(val: JsValue) -> Self {
        let (sender, receiver) = spsc::SharedChannel::from(val).split();
        Channel {
            sender: Some(sender),
            receiver: Some(receiver),
        }
    }

    pub fn replica(&self) -> JsValue {
        self.receiver.as_ref().unwrap().0.clone().into()
    }

    pub fn run(&mut self, scope: web_sys::DedicatedWorkerGlobalScope) -> Result<(), JsValue> {
        console_error_panic_hook::set_once();
        let receiver = self.receiver.take().unwrap();
        let (sender, mut rcvr) = oneshot::channel();
        let fut = async move {
            let mut model_id_counter = 0u32;
            let mut models = HashMap::new();
            let mut previous_tokens = executor::queued_tokens();
            loop {
                crate::sys::task::yield_now().await;
                let tokens = executor::queued_tokens();
                // stasis detection
                // current thesis is that if there's nothing queued or it's the same tokens
                // as in the previous iteration of the loop, it means we need new input to
                // resolve any of the futures
                let timeout = if executor::queued_tasks() == 0 || tokens == previous_tokens {
                    Some(std::time::Duration::from_secs(1))
                } else {
                    None
                };
                previous_tokens = tokens;
                match receiver.recv(timeout) {
                    Err(_) => {
                        let _ = sender.send(receiver);
                        break;
                    }
                    Ok(None) => {}
                    Ok(Some(Request {
                        id,
                        variant: Variant::Info,
                    })) => {
                        let response = js_sys::Map::new();
                        response.set(&"id".into(), &id.into());
                        response.set(&"status".into(), &"running".into());
                        let _ =
                            scope.post_message(&js_sys::Object::from_entries(&response).unwrap());
                    }
                    Ok(Some(Request {
                        id,
                        variant: Variant::CreateModel { xml },
                    })) => match bpmn::parse(&xml) {
                        Ok(doc) => {
                            let model_id = model_id_counter;
                            model_id_counter += 1;
                            models.insert(model_id, Model::new(doc).spawn().await);
                            let response = js_sys::Map::new();
                            response.set(&"id".into(), &id.into());
                            response.set(&"model_id".into(), &model_id.into());
                            let _ = scope
                                .post_message(&js_sys::Object::from_entries(&response).unwrap());
                        }
                        Err(err) => {
                            let response = js_sys::Map::new();
                            response.set(&"id".into(), &id.into());
                            response.set(
                                &"error".into(),
                                &format!("error parsing xml: {}", err).into(),
                            );
                            let _ = scope
                                .post_message(&js_sys::Object::from_entries(&response).unwrap());
                        }
                    },
                    Ok(Some(Request {
                        id,
                        variant: Variant::ListProcesses { model_id },
                    })) => match models.get(&model_id) {
                        None => {
                            let response = js_sys::Map::new();
                            response.set(&"id".into(), &id.into());
                            response.set(&"error".into(), &"model not found".into());
                            let _ = scope
                                .post_message(&js_sys::Object::from_entries(&response).unwrap());
                        }
                        Some(model) => {
                            let processes = model.processes().await.unwrap();
                            let map = js_sys::Map::new();
                            for (index, process) in processes.iter().enumerate() {
                                let id = match process.element().id() {
                                    None => JsValue::from(index as u32),
                                    Some(id) => JsValue::from(id),
                                };
                                map.set(&JsValue::from(index as u32), &id);
                            }
                            let response = js_sys::Map::new();
                            response.set(&"id".into(), &id.into());
                            response.set(
                                &"processes".into(),
                                &js_sys::Object::from_entries(&map).unwrap(),
                            );
                            let _ = scope
                                .post_message(&js_sys::Object::from_entries(&response).unwrap());
                        }
                    },
                    Ok(Some(Request {
                        id,
                        variant:
                            Variant::StartProcess {
                                model_id,
                                process_id,
                            },
                    })) => match models.get(&model_id) {
                        None => {
                            let response = js_sys::Map::new();
                            response.set(&"id".into(), &id.into());
                            response.set(&"error".into(), &"model not found".into());
                            let _ = scope
                                .post_message(&js_sys::Object::from_entries(&response).unwrap());
                        }
                        Some(model) => {
                            let processes = model.processes().await.unwrap();
                            match processes
                                .iter()
                                .enumerate()
                                .find(|(index, _)| *index as u32 == process_id)
                            {
                                None => {
                                    let response = js_sys::Map::new();
                                    response.set(&"id".into(), &id.into());
                                    response.set(&"error".into(), &"process not found".into());
                                    let _ = scope.post_message(
                                        &js_sys::Object::from_entries(&response).unwrap(),
                                    );
                                }
                                Some((_, process)) => {
                                    let result = process.start().await;
                                    match result {
                                        Ok(()) => {
                                            let response = js_sys::Map::new();
                                            response.set(&"id".into(), &id.into());
                                            response.set(&"started".into(), &JsValue::TRUE);
                                            let _ = scope.post_message(
                                                &js_sys::Object::from_entries(&response).unwrap(),
                                            );
                                        }
                                        Err(err) => {
                                            let response = js_sys::Map::new();
                                            response.set(&"id".into(), &id.into());
                                            response.set(&"started".into(), &JsValue::FALSE);
                                            response
                                                .set(&"error".into(), &format!("{}", err).into());

                                            let _ = scope.post_message(
                                                &js_sys::Object::from_entries(&response).unwrap(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Ok(Some(Request {
                        id,
                        variant:
                            Variant::SubscribeProcessLog {
                                model_id,
                                process_id,
                            },
                    })) => match models.get(&model_id) {
                        None => {
                            let response = js_sys::Map::new();
                            response.set(&"id".into(), &id.into());
                            response.set(&"error".into(), &"model not found".into());
                            let _ = scope
                                .post_message(&js_sys::Object::from_entries(&response).unwrap());
                        }
                        Some(model) => {
                            let processes = model.processes().await.unwrap();
                            match processes
                                .iter()
                                .enumerate()
                                .find(|(index, _)| *index as u32 == process_id)
                            {
                                None => {
                                    let response = js_sys::Map::new();
                                    response.set(&"id".into(), &id.into());
                                    response.set(&"error".into(), &"process not found".into());
                                    let _ = scope.post_message(
                                        &js_sys::Object::from_entries(&response).unwrap(),
                                    );
                                }
                                Some((_, process)) => {
                                    let mut log_receiver = process.log_receiver();
                                    let worker_scope = scope.clone();
                                    executor::spawn(async move {
                                        loop {
                                            if let Ok(message) = log_receiver.recv().await {
                                                let notification = js_sys::Map::new();
                                                notification.set(&"id".into(), &id.into());
                                                notification.set(
                                                    &"data".into(),
                                                    &JsValue::from_serde(&message).unwrap(),
                                                );
                                                let _ = worker_scope.post_message(
                                                    &js_sys::Object::from_entries(&notification)
                                                        .unwrap(),
                                                );
                                            } else {
                                                break;
                                            }
                                        }
                                    });
                                    let response = js_sys::Map::new();
                                    response.set(&"id".into(), &id.into());
                                    response.set(&"subscribed".into(), &JsValue::TRUE);
                                    let _ = scope.post_message(
                                        &js_sys::Object::from_entries(&response).unwrap(),
                                    );
                                }
                            }
                        }
                    },
                }
            }
        };
        use wasm_rs_async_executor::single_threaded as executor;
        executor::run(Some(executor::spawn(fut)));
        self.receiver.replace(rcvr.try_recv().unwrap());
        Ok(())
    }

    pub fn sender(&mut self) -> Result<Sender, JsValue> {
        match self.sender.take() {
            Some(sender) => Ok(Sender(sender)),
            None => Err("sender is already taken".to_string().into()),
        }
    }
}

#[wasm_bindgen]
pub struct Sender(spsc::Sender<Request>);

#[wasm_bindgen]
impl Sender {
    pub fn info(&self, id: u32) -> Result<(), JsValue> {
        self.0.send(&Request {
            id,
            variant: Variant::Info,
        })
    }

    #[wasm_bindgen(js_name = "createModel")]
    pub fn create_model(&self, xml: String, id: u32) -> Result<(), JsValue> {
        self.0.send(&Request {
            id,
            variant: Variant::CreateModel { xml },
        })
    }

    #[wasm_bindgen(js_name = "processes")]
    pub fn list_processes(&self, model_id: u32, id: u32) -> Result<(), JsValue> {
        self.0.send(&Request {
            id,
            variant: Variant::ListProcesses { model_id },
        })
    }

    #[wasm_bindgen(js_name = "startProcess")]
    pub fn start_process(&self, model_id: u32, process_id: u32, id: u32) -> Result<(), JsValue> {
        self.0.send(&Request {
            id,
            variant: Variant::StartProcess {
                model_id,
                process_id,
            },
        })
    }

    #[wasm_bindgen(js_name = "subscribeToProcessLog")]
    pub fn subscribe_process_log(
        &self,
        model_id: u32,
        process_id: u32,
        id: u32,
    ) -> Result<(), JsValue> {
        self.0.send(&Request {
            id,
            variant: Variant::SubscribeProcessLog {
                model_id,
                process_id,
            },
        })
    }
}
