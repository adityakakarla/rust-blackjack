use std::{thread, time};

pub fn wait_n_seconds(n: u64){
    let one_second = time::Duration::from_secs(n);
    thread::sleep(one_second);
}