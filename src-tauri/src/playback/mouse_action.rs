pub mod mouse {
    use rdev::{simulate, EventType};
    use std::{
        thread,
        time::Duration,
    };
   

    const INTERVAL: Duration = Duration::from_millis(10);

    //鼠标移动操作确认
    pub fn send_move(event_type: &EventType, wait_time: u128) {
        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
        match simulate(event_type) {
            Ok(()) => (),
            Err(_simulate_error) => {
                println!("复现鼠标操作 {:?}出错", event_type);
            }
        }
        thread::sleep(wait_duration);
    }
    //鼠标点击操作确认
    pub fn send(event_type: &EventType) {
        match simulate(event_type) {
            Ok(()) => (),
            Err(_simulate_error) => {
                println!("复现鼠标操作 {:?}出错", event_type);
            }
        }
        thread::sleep(INTERVAL);
    }
}
