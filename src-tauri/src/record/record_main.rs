pub mod record_main {
    use device_query::{DeviceQuery, DeviceState, Keycode};
    use rdev::listen;
    use std::process::Command;
    use std::{fs::OpenOptions, io::Write, thread, thread::sleep, time::Duration};

    use crate::record::mouse_monitor;

    use crate::MOUSE_MOVE_TIME;
    use crate::SCREEN_SHOT_FLAG;
    use crate::START_TIME;
    use crate::MOUSE_THREAD_FLAG;

    pub fn start_record() {
        let mut status = "init";
        let device_state = DeviceState::new();
        println!("请按下F1开始");
        //设定开始按键
        loop {
            let start_keys: Vec<Keycode> = device_state.get_keys();
            if start_keys.is_empty() || start_keys[0] != Keycode::F1 {
                continue;
            } else {
                // let duration = START_TIME.lock().unwrap().elapsed().as_millis();
                println!("程序开始");
                println!("F2启动截图功能");
                status = "started";
                sleep(Duration::from_millis(200));
                let start_time = START_TIME.lock().unwrap().elapsed().as_millis();
                *MOUSE_MOVE_TIME.lock().unwrap() = Some(start_time);
                let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();
                *mouse_flag = false;
                break;
            }
        }

        //键盘监听文件
        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("./result/record.txt")
            .unwrap();

        // 鼠标监听线程
        thread::spawn(|| {
            if let Err(error) = listen(mouse_monitor::mouse::callback) {
                println!("Error: {:?}", error);
            }
        });

        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if keys.len() != 0 && keys[0] == Keycode::F2 {
                sleep(Duration::from_millis(300));
                //截图事件F2触发
                println!("进入截图功能");
                let mut screen_flag = SCREEN_SHOT_FLAG.lock().unwrap();
                *screen_flag = true;
                Command::new("textshot")
                    .spawn()
                    .expect("无法启动 textshot 命令");
                continue;
            } else if keys.len() != 0 && keys[0] != Keycode::F1 {
                let duration_key = START_TIME.lock().unwrap().elapsed().as_millis();
                println!("{}ms,捕捉到键盘输入{:?}", duration_key, keys[0]);
                let output = format!("{},key,{:?}\n", duration_key, keys[0]);
                save_file.write_all(output.as_bytes()).unwrap();
                sleep(Duration::from_millis(175));
                continue;
            // }else if keys.len() == 0 || keys[0] != Keycode::F1 {
            } else if keys.len() == 0 {
                continue;
            }
            let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();
            *mouse_flag = true;
            if status == "started" {
                println!(
                    "\n经过了：{}毫秒",
                    START_TIME.lock().unwrap().elapsed().as_millis()
                );
                return;
            }
        }
    }
}
