pub mod mouse {
    use rdev::{Button, Event, EventType};
    use std::{
        fs::{File, OpenOptions},
        io::Write,
    };
    use tauri::window;
    use tauri::{Manager, Window};
    use serde::Serialize;

    use crate::record::screen_shot;
    use crate::MOUSE_BEFORE_PRESS;
    use crate::MOUSE_MOVE_TIME;
    use crate::MOUSE_PATH;
    use crate::MOUSE_THREAD_FLAG;
    use crate::PAUSE_TIME;
    use crate::SCREEN_PRESS;
    use crate::SCREEN_SHOT_FLAG;
    use crate::START_TIME;

    #[derive(Clone, serde::Serialize)]
    struct Payload {
        message: f64,
    }

    pub fn callback(event: Event, window: &Window) {
        let mut  y_axis:f64 = 0.0; // y坐标
        if let EventType::MouseMove { x, y } = event.event_type {
            y_axis = y;
        }
        
        window
        .emit(
            "y-axis",
            Payload {
                message: y_axis,
            },
        )
        .unwrap();

        let mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();
        if *mouse_flag == true {
            return;
        }
        let mouse_path = MOUSE_PATH.lock().unwrap();
        let now_dir = mouse_path.clone();
        let last_move_time = *MOUSE_MOVE_TIME.lock().unwrap();
        let pause_time = PAUSE_TIME.lock().unwrap();
        let duration = START_TIME.lock().unwrap().elapsed().as_millis() - *pause_time;

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!(
                "../Automated-testing/result/{}/record.txt",
                now_dir
            ))
            .unwrap();
        let mut screen_flag = SCREEN_SHOT_FLAG.lock().unwrap();
        if *screen_flag == true {
            //调用系统截图工具
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    //记录最后坐标点用于在鼠标点击动作移动动作后标注
                    let mut mouse_before_press = MOUSE_BEFORE_PRESS.lock().unwrap();
                    *mouse_before_press = (x, y);
                }
                EventType::ButtonPress(button) => screen_record_press(duration, button, file),
                EventType::ButtonRelease(button) => {
                    screen_record_release(duration, button, file, now_dir);
                    *screen_flag = false;
                }
                _ => {}
            }
        } else {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    let mut mouse_before_press = MOUSE_BEFORE_PRESS.lock().unwrap();
                    *mouse_before_press = (x, y);
                    //计算距离上一次记录鼠标移动事件的时间
                    let last_move_time = last_move_time.unwrap();
                    let elapsed = duration - last_move_time;
                    if elapsed > 200 {
                        record_move(duration, x.to_string(), y.to_string(), file);
                        *MOUSE_MOVE_TIME.lock().unwrap() = Some(duration);
                    }
                }
                EventType::ButtonPress(button) => record_press(duration, button, file),
                EventType::ButtonRelease(button) => record_release(duration, button, file),
                _ => {}
            }
        }
    }

    //记录用户鼠标移动
    pub fn record_move(time: u128, x: String, y: String, mut file: File) {
        let mut val = String::new();
        val.push_str(&time.to_string());
        val.push_str("ms");
        val.push(',');
        val.push_str("move");
        val.push(',');
        val.push_str(&x);
        val.push(',');
        val.push_str(&y);
        val.push('\n');

        file.write_all(val.as_bytes()).unwrap();
        println!("{}ms，指针记录值: x {},y {}", time, x, y);
    }

    //记录用户鼠标按下
    pub fn record_press(time: u128, button: Button, mut file: File) {
        let mouse_before_press = MOUSE_BEFORE_PRESS.lock().unwrap();
        let (x, y) = *mouse_before_press;
        let mut val = String::new();
        match button {
            Button::Left => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",press,Left");
                val.push(',');
                val.push_str(&x.to_string());
                val.push(',');
                val.push_str(&y.to_string());
                val.push('\n')
            }
            Button::Right => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",press,Right");
                val.push(',');
                val.push_str(&x.to_string());
                val.push(',');
                val.push_str(&y.to_string());
                val.push('\n')
            }
            Button::Middle => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",press,Middle");
                val.push(',');
                val.push_str(&x.to_string());
                val.push(',');
                val.push_str(&y.to_string());
                val.push('\n')
            }
            _ => {}
        }
        file.write_all(val.as_bytes()).unwrap();
    }

    //记录用户鼠标弹起
    pub fn record_release(time: u128, button: Button, mut file: File) {
        let mut val = String::new();
        match button {
            Button::Left => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",release,Left\n")
            }
            Button::Right => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",release,Right\n")
            }
            Button::Middle => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",release,Middle\n")
            }
            _ => {}
        }
        file.write_all(val.as_bytes()).unwrap();
    }

    //记录截图鼠标按下
    pub fn screen_record_press(time: u128, button: Button, mut file: File) {
        let mouse_before_press = MOUSE_BEFORE_PRESS.lock().unwrap();
        let (x, y) = *mouse_before_press;
        let mut scrren_press = SCREEN_PRESS.lock().unwrap();
        *scrren_press = (x, y);
        let mut val = String::new();
        match button {
            Button::Left => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",screen_press,Left");
                val.push(',');
                val.push_str(&x.to_string());
                val.push(',');
                val.push_str(&y.to_string());
                val.push('\n')
            }
            _ => {}
        }
        file.write_all(val.as_bytes()).unwrap();
    }

    //记录截图鼠标弹起
    pub fn screen_record_release(time: u128, button: Button, mut file: File, now_dir: String) {
        //读取按下坐标
        let screen_press = SCREEN_PRESS.lock().unwrap();
        let (b_x, b_y) = *screen_press;
        let mouse_before_press = MOUSE_BEFORE_PRESS.lock().unwrap();
        let (x, y) = *mouse_before_press;
        let mut val = String::new();
        match button {
            Button::Left => {
                val.push_str(&time.to_string());
                val.push_str("ms");
                val.push_str(",screen_release,Left");
                val.push(',');
                val.push_str(&x.to_string());
                val.push(',');
                val.push_str(&y.to_string());
                val.push('\n')
            }
            _ => {}
        }
        file.write_all(val.as_bytes()).unwrap();

        screen_shot::screen::screenshot(b_x, b_y, x, y, time, now_dir);
    }
}
