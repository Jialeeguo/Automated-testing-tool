pub mod playback_main {
    use crate::playback::keyboard_action::keyboard;
    use crate::playback::mouse_action::mouse;
    use crate::playback::screen_shot_action::screen;
    use chrono::prelude::*;
    use enigo::{Enigo, Key, KeyboardControllable};
    use rdev::{Button, EventType};
    use serde::Deserialize;
    use std::fmt::format;
    use std::path::Path;
    use std::{
        fmt::write,
        fs::{self, File, OpenOptions},
        io::{BufRead, BufReader, Read, Write},
        thread,
        time::Duration,
    };
    use tauri::window;
    use tauri::{Manager, Window};

    use crate::LAST_ACTION_TIME;
    use crate::SCREEN_PRESS;

    #[derive(Clone, serde::Serialize)]
    struct ResultListen {
        message: String,
    }

    #[tauri::command]
    pub fn playback_main(file_path: String, lang: String, window: Window) {
        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // 清空文件内容
            .open(format!("{}/record_result.txt", file_path))
            .unwrap();
        save_file.set_len(0).unwrap();
        println!("所选语言：{}\n", lang);
        let now_dir = String::from(file_path);
        *LAST_ACTION_TIME.lock().unwrap() = 0;
        let file = match File::open(format!("{}/record.txt", now_dir.clone())) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                return;
            }
        };
        let local: DateTime<Local> = Local::now();
        let back_time = local.format("%Y-%m-%d %H-%M-%S").to_string();
        let log_file = format!("{}/generated_{}.html", now_dir, back_time.clone());
        let mut html_file = File::create(log_file.clone()).expect("Failed to create HTML file");
        html_file
            .write_all(
                format!(
                    "<html><head><title>{}脚本回放日志 时间：{}</title></head><body>",
                    now_dir, back_time
                )
                .as_bytes(),
            )
            .expect("Failed to write HTML to file");
        html_file.flush().expect("Failed to flush HTML file");
        drop(html_file);
        let read_script = BufReader::new(file);
        for line in read_script.lines() {
            let instruct = line.unwrap();
            let arr: Vec<&str> = instruct.split(",").collect();
            let time: u128 = arr[0].parse().unwrap();
            let wait_time = time - *LAST_ACTION_TIME.lock().unwrap();
            *LAST_ACTION_TIME.lock().unwrap() = time;
            let action = arr[1];
            let content = arr[2];
            let mut enigo = Enigo::new();
            match action {
                "move" => {
                    let x = content.parse::<f64>().unwrap();
                    let y = arr[3].parse::<f64>().unwrap();
                    mouse::send_move(&EventType::MouseMove { x, y }, wait_time)
                }
                "press" => match content {
                    "Left" => {
                        let x = arr[3].parse::<f64>().unwrap();
                        let y = arr[4].parse::<f64>().unwrap();
                        mouse::send_move(&EventType::MouseMove { x, y }, 1);
                        mouse::send(&EventType::ButtonPress(Button::Left))
                    }
                    "Right" => mouse::send(&EventType::ButtonPress(Button::Right)),
                    "Middle" => mouse::send(&EventType::ButtonPress(Button::Middle)),
                    _ => {}
                },
                "release" => match content {
                    "Left" => mouse::send(&EventType::ButtonRelease(Button::Left)),
                    "Right" => mouse::send(&EventType::ButtonRelease(Button::Right)),
                    "Middle" => mouse::send(&EventType::ButtonRelease(Button::Middle)),
                    _ => {}
                },
                "key_down" => {
                    //判断键盘content按下是什么内容
                    //如果是单个字符，那就变成小写然后模拟输入
                    if content.len() == 1 && content.chars().all(|c| c.is_ascii_uppercase()) {
                        let lowercase_char = content.chars().next().unwrap().to_ascii_lowercase();
                        enigo.key_down(Key::Layout(lowercase_char));
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        thread::sleep(wait_duration);
                    } else {
                        //如果是其他按键，到哈希表找对应按键值，执行操作
                        let key = match keyboard::get_key(content) {
                            Some(k) => k,
                            None => return,
                        };
                        enigo.key_down(key);
                        // enigo.key_up(key);
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        thread::sleep(wait_duration);
                    }
                },
                "key_up" => {
                    //判断键盘content抬起是什么内容
                    //如果是单个字符，那就变成小写然后模拟输入
                    if content.len() == 1 && content.chars().all(|c| c.is_ascii_uppercase()) {
                        let lowercase_char = content.chars().next().unwrap().to_ascii_lowercase();
                        enigo.key_up(Key::Layout(lowercase_char));
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        thread::sleep(wait_duration);
                    } else {
                        //如果是其他按键，到哈希表找对应按键值，执行操作
                        let key = match keyboard::get_key(content) {
                            Some(k) => k,
                            None => return,
                        };
                        // enigo.key_down(key);
                        enigo.key_up(key);
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        thread::sleep(wait_duration);
                    }
                }
                "screen_press" => match content {
                    "Left" => {
                        let x = arr[3].parse::<f64>().unwrap();
                        let y = arr[4].parse::<f64>().unwrap();
                        let mut scrren_press = SCREEN_PRESS.lock().unwrap();
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        *scrren_press = (x, y);
                        thread::sleep(wait_duration);
                    }
                    _ => {}
                },
                "screen_release" => match content {
                    "Left" => {
                        let screen_press = SCREEN_PRESS.lock().unwrap();
                        let (b_x, b_y) = *screen_press;
                        let x = arr[3].parse::<f64>().unwrap();
                        let y = arr[4].parse::<f64>().unwrap();
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        screen::screenshot(
                            b_x,
                            b_y,
                            x,
                            y,
                            time,
                            now_dir.clone(),
                            lang.clone(),
                            log_file.clone(),
                        );
                        thread::sleep(wait_duration);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        println!("record结果传前端");
        window
            .emit(
                "result_listen",
                ResultListen {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
    }

    //测试用例是否通过状态
    #[tauri::command]
    pub fn playback_confirm(file_path: String, playback_result: String) {
        let file_path = format!("{}/record_result.txt", file_path);
        let result_text = std::fs::read_to_string(&file_path).unwrap_or_default();
        println!("result_text:{}\n", result_text);
        let mut process_lines = vec![];
        for line in result_text.lines() {
            if !line.contains("回放结果为：") {
                process_lines.push(line);
            }
        }
        println!("process_lines:{:?}\n", process_lines);
        let output_text = process_lines.join("\n");
        println!("output_text{}", output_text);
        std::fs::write(&file_path, output_text).unwrap_or_default();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)
            .expect("无法打开文件");
        writeln!(file, "\n回放结果为：{}\n", playback_result).unwrap_or_default();
    }

    //选择的回放文件夹是否存在
    #[tauri::command]
    pub fn dir_confirm(file_path: String) -> Vec<Vec<String>> {
        let file = File::open(format!("{}/record.txt", file_path));
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut script: Vec<Vec<String>> = Vec::new();
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let words: Vec<String> =
                            line.trim().split_whitespace().map(String::from).collect();
                        script.push(words);
                    }
                }

                script
            }
            Err(error) => {
                println!("Error opening file: {}", error);
                let content: Vec<Vec<String>> = Vec::new();
                content
            }
        }
    }
    #[tauri::command]
    pub fn check_file_exists(file_path: String) -> bool {
        let file_path = format!("{}/record.txt", file_path);
        let path = Path::new(&file_path);
        let b = path.exists() && path.is_file();
        println!("结果{}", b);
        b
    }
    #[tauri::command]
    pub fn return_record_result(file_path: String) -> String {
        let file = File::open(format!("{}/record_result.txt", file_path));
        match file {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut content = String::new();
                // 将文件内容读取到字符串中
                reader
                    .read_to_string(&mut content)
                    .expect("Error reading file");

                content
            }
            Err(error) => {
                println!("Error opening file: {}", error);
                let content = format!("");
                content
            }
        }
    }

    #[tauri::command]
    pub fn search_test_dir(file_path:String) -> Vec<String> {
        let mut file = Vec::new();

        if let Ok(entries) = fs::read_dir(Path::new(&file_path)) {
            for entry in entries{
                if let Ok(entry) = entry{
                    if let Ok(file_name) = entry.file_name().into_string() {
                        file.push(file_name);
                    }
                }
            }
        }
        file
    }
}
