pub mod record_main {
    use crate::record::mouse_monitor;
    use crate::MOUSE_MOVE_TIME;
    use crate::MOUSE_PATH;
    use crate::MOUSE_THREAD_FLAG;
    use crate::MOUSE_THREAD_START;
    use crate::NOW_DIR;
    use crate::SCREEN_SHOT_FLAG;
    use crate::SHOULD_STOP;
    use crate::SHOULD_STOP_KEYBOARD;
    use crate::START_TIME;
    use crate::TEXTSHOT_ACTION_TIME;
    use chrono::prelude::*;
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;
    use device_query::{DeviceQuery, DeviceState, Keycode};
    use rdev::listen;
    use std::process::Command;
    use tauri::window;
    use tauri::{Manager, Window};
    // use std::sync::{Arc, Mutex};
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    };
    use std::time::Instant;
    use std::{fs::OpenOptions, io::Write, thread, thread::sleep, time::Duration};
    #[derive(Clone, serde::Serialize)]
    struct Payload {
        message: String,
    }
    #[tauri::command]
    pub async fn start_record(mut recordstart: bool) {
        let mut status = "init";
        let device_state = DeviceState::new();
        //存储工作目录文件夹名
        let mut now_dir = String::new();
        if (recordstart == true) {
            return;
        }
        //设定开始按键
        loop {
            // let start_keys: Vec<Keycode> = device_state.get_keys();

            //还没想好逻辑
            // let should_stop = {
            //     let stop_flag = SHOULD_STOP.lock().unwrap();
            //     *stop_flag
            // };

            // if should_stop {
            //     println!("录制被终止");
            //     break;
            // }
            sleep(Duration::from_millis(200));
            println!("程序开始");
            println!("F2启动截图功能");

            //获取系统时间，创建当前项目文件夹
            let local: DateTime<Local> = Local::now();
            now_dir = local.format("%Y-%m-%d %H-%M-%S").to_string();
            std::fs::create_dir_all(format!("../Automated-testing/result/{}", now_dir)).unwrap();

            let mut global_now_dir = NOW_DIR.lock().unwrap();
            *global_now_dir = now_dir.clone();

            let mut mouse_path = MOUSE_PATH.lock().unwrap();
            *mouse_path = now_dir.clone();

            //开启计时
            status = "started";
            //重置计时时间
            let mut start_time = START_TIME.lock().unwrap();
            *start_time = Instant::now();

            *MOUSE_MOVE_TIME.lock().unwrap() = Some(start_time.elapsed().as_millis());
            //鼠标监听标志
            let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();

            *mouse_flag = false;

            break;
        }

        //键盘监听文件

        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!(
                "../Automated-testing/result/{}/record.txt",
                now_dir
            ))
            .unwrap();
        // 鼠标监听线程
        let mut mouse_flag = MOUSE_THREAD_START.lock().unwrap();
        if *mouse_flag == false {
            *mouse_flag = true;

            // 使用 Arc<AtomicBool> 共享键盘监听标志
            let should_stop_keyboard = SHOULD_STOP_KEYBOARD.clone();

            thread::spawn(move || {
                if let Err(error) =
                    listen(move |event| mouse_monitor::mouse::callback(event, recordstart))
                {
                    println!("Error: {:?}", error);
                }
            });
        }

        loop {
            let should_stop = {
                let stop_flag = SHOULD_STOP.lock().unwrap();
                *stop_flag
            };

            if should_stop {
                println!("录制被终止");
                break;
            }
            let keys: Vec<Keycode> = device_state.get_keys();
            if (keys.len() != 0 && keys[0] != Keycode::F1) {
                let duration_key = START_TIME.lock().unwrap().elapsed().as_millis();
                println!("{}ms,捕捉到键盘输入{:?}", duration_key, keys[0]);
                let output = format!("{},key,{:?}\n", duration_key, keys[0]);
                save_file.write_all(output.as_bytes()).unwrap();
                sleep(Duration::from_millis(175));
                continue;
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
                // std::thread::sleep(std::time::Duration::from_secs(3));
                // mouse_stop_flag = true;
                // println!("flag当前是3：{}", mouse_stop_flag);

                // handle.join().unwrap();
                return;
            }
        }
    }
    //点击终止录制后函数
    #[tauri::command]
    pub async fn start_screen(mut clickButton: bool, window: Window) {
        loop {
            let mut now_dir = String::new();
            let device_state = DeviceState::new();
            let keys: Vec<Keycode> = device_state.get_keys();

            let global_now_dir = NOW_DIR.lock().unwrap();
            now_dir = global_now_dir.clone();

            let should_stop_keyboard = SHOULD_STOP_KEYBOARD.clone();

            if should_stop_keyboard.load(Ordering::Relaxed) {
                println!("键盘监听线程结束");
                break;
            }
            let mut save_file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(format!(
                    "../Automated-testing/result/{}/record.txt",
                    now_dir
                ))
                .unwrap();
            if (keys.len() != 0 && keys[0] == Keycode::F2) || clickButton == true {
                println!("传进来第一次的clickButton是{}", clickButton);
                clickButton = false;
                sleep(Duration::from_millis(300));
                //截图事件F2触发
                println!("进入截图功能");
                println!("请等待几秒，正在提取图片文字...");
                {
                    let mut screen_flag = SCREEN_SHOT_FLAG.lock().unwrap();
                    *screen_flag = true;
                }
                let output = Command::new("C:/Users/trookie/venv/Scripts/textshot")
                    .arg("eng+chi_sim")
                    .output()
                    .expect("无法启动 textshot 命令");

                if output.status.success() {
                    let textshot_time = *TEXTSHOT_ACTION_TIME.lock().unwrap();

                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(format!(
                            "../Automated-testing/result/{}/textshot_{}.txt",
                            now_dir,
                            textshot_time.to_string()
                        ))
                        .expect("无法打开文件");

                    let output_confirm = String::from_utf8_lossy(&output.stdout);
                    //判断是否没有文字，如果是就不写入了
                    if output_confirm
                        .contains("ERROR: Unable to read text from image, did not copy")
                    {
                    } else {
                        // let output_bytes = &output.stdout;
                        // let output_without_info = &output_bytes[14..output_bytes.len() - 20];
                        let mut output_without_info: ClipboardContext =
                            ClipboardProvider::new().unwrap();
                        let mut clipboard_content = output_without_info.get_contents().unwrap();
                        clipboard_content.push('\n');
                        file.write_all(clipboard_content.as_bytes())
                            .expect("提取文字命令写入文件失败");
                    }
                    window
                        .emit(
                            "event-name",
                            Payload {
                                message: "Tauri is awesome!".into(),
                            },
                        )
                        .unwrap();
                    println!("提取文字执行成功！请继续操作。");
                    
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    println!("提取文字命令执行失败: {}", error);
                }

                continue;
            } else if (keys.len() != 0 && keys[0] != Keycode::F1) {
                sleep(Duration::from_millis(175));
                continue;
            } else if keys.len() == 0 {
                continue;
            }

            let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();

            *mouse_flag = true;
        }
    }
}
