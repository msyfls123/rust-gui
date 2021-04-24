use std::thread;
use std::sync::{Arc};
use std::time;
use tokio::sync::{Mutex};
use tokio::sync::mpsc::{UnboundedReceiver};
use druid::{Target};

use crate::types::selector::{DAY_DATA, CONCURRENCY_COUNT};
use crate::utils::api::get_data;

enum UpdateConcurrencyMethod {
  PlusOne,
  MinusOne,
}

async fn update_concurrency(
  sink: &Arc<druid::ExtEventSink>,
  current: &Arc<Mutex<u32>>,
  method: UpdateConcurrencyMethod,
) {
  use UpdateConcurrencyMethod::{PlusOne, MinusOne};
  let mut locked_concurrency = current.lock().await;
  match method {
    PlusOne => *locked_concurrency += 1,
    MinusOne => *locked_concurrency -= 1,
  }
  println!("并发 == {}", locked_concurrency);
  sink
    .submit_command(CONCURRENCY_COUNT, *locked_concurrency, Target::Auto)
    .expect("send concurrency failed");
}

pub fn request_day(
  initial_concurrency: u32,
  rx: Arc<Mutex<UnboundedReceiver<u32>>>,
  event_sink: Arc<druid::ExtEventSink>,
) {
    println!("init request day event handler");
    let cloned = rx.clone();
    let concurrency = Arc::new(Mutex::new(initial_concurrency));
    tokio::spawn(async move {
        let mut locked = cloned.lock().await;
        while let res = locked.recv().await {
            match res {
                Some(day) => {
                    println!("{}", day);
                    let cloned_sink = Arc::clone(&event_sink);
                    let cloned_concurrency = Arc::clone(&concurrency);
                    tokio::spawn(async move {
                        update_concurrency(&cloned_sink, &cloned_concurrency, UpdateConcurrencyMethod::PlusOne).await;
                        let res = get_data(day).await.unwrap();
                        thread::sleep(time::Duration::from_millis(1000));
                        let day_data = format!("{:?}", res);
                        cloned_sink
                            .submit_command(DAY_DATA, day_data, Target::Auto)
                            .expect("uhhh");
                        update_concurrency(&cloned_sink, &cloned_concurrency, UpdateConcurrencyMethod::MinusOne).await;
                    });
                    
                },
                _ => eprintln!("empty"),
            }
        }
    });
}
