pub mod mouse {
    use rdev::{simulate, EventType};
    use std::{
        thread,
        time::Duration,
    };

    /// 根据帧数计算间隔时间
    const INTERVAL: Duration = Duration::from_millis(10);

    //鼠标移动操作确认
    pub fn send_move(event_type: &EventType, wait_time: u128) {
        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
        match simulate(event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("We could not send {:?}", event_type);
            }
        }
        //每次暂停的间隔，本次时间减去上次时间
        thread::sleep(wait_duration);
    }
    //鼠标点击操作确认
    pub fn send(event_type: &EventType) {
        match simulate(event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("We could not send {:?}", event_type);
            }
        }
        //每次暂停的间隔，本次时间减去上次时间
        thread::sleep(INTERVAL);
    }
}
