pub mod playback_main {
    use enigo::{Enigo, Key, KeyboardControllable};
    use rdev::{Button, EventType};
    use std::{
        fmt::write,
        fs::{self, File, OpenOptions},
        io::{BufRead, BufReader, Write},
        thread,
        time::Duration,
    };

    use crate::playback::keyboard_action::keyboard;
    use crate::playback::mouse_action::mouse;
    use crate::playback::screen_shot_action::screen;

    use crate::LAST_ACTION_TIME;
    use crate::SCREEN_PRESS;
    #[tauri::command]
    pub fn playback_main(file_path: String, lang: String) {
        println!("所选语言：{}\n", lang);
        let now_dir = String::from(file_path);
        *LAST_ACTION_TIME.lock().unwrap() = 0;
        let file = match File::open(format!("{}/record.txt", now_dir)) {
            Ok(f) => f,
            Err(e) => {
                // 打印错误信息并返回
                eprintln!("Error opening file: {}", e);
                return;
            }
        };
        let read_script = BufReader::new(file);
        for line in read_script.lines() {
            let instruct = line.unwrap();
            let arr: Vec<&str> = instruct.split(",").collect();
            let time: u128 = arr[0].parse().unwrap();
            let wait_time = time - *LAST_ACTION_TIME.lock().unwrap();
            *LAST_ACTION_TIME.lock().unwrap() = time;
            let action = arr[1];
            let content = arr[2];
            //键盘复现
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
                "key" => {
                    //判断键盘content是什么内容（待优化）
                    //如果是单个字符，那就变成小写然后模拟输入
                    if content.len() == 1 && content.chars().all(|c| c.is_ascii_uppercase()) {
                        let lowercase_char = content.chars().next().unwrap().to_ascii_lowercase();
                        enigo.key_click(Key::Layout(lowercase_char));
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        thread::sleep(wait_duration);
                    } else {
                        //如果是其他按键，到哈希表找对应按键值，执行操作
                        let key = match keyboard::get_key(content) {
                            Some(k) => k,
                            None => return,
                        };
                        enigo.key_down(key);
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
                        //读取按下坐标
                        let screen_press = SCREEN_PRESS.lock().unwrap();
                        let (b_x, b_y) = *screen_press;
                        //读取抬起坐标
                        let x = arr[3].parse::<f64>().unwrap();
                        let y = arr[4].parse::<f64>().unwrap();
                        let wait_duration = Duration::from_millis(wait_time.try_into().unwrap());
                        screen::screenshot(b_x, b_y, x, y, time, now_dir.clone(), lang.clone());
                        thread::sleep(wait_duration);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    //测试用例是否通过状态
    #[tauri::command]
    pub fn playback_confirm(file_path: String, playback_result: String) {
        let file_path = format!("{}/record_result.txt", file_path);

        // 读取文件内容，如果文件不存在或为空，则提供默认值为空字符串
        let result_text = std::fs::read_to_string(&file_path).unwrap_or_default();
        println!("result_text:{}\n", result_text);

        // 处理文件内容
        let mut process_lines = vec![];
        for line in result_text.lines() {
            if !line.contains("回放结果为：") {
                process_lines.push(line);
            }
        }
        println!("process_lines:{:?}\n", process_lines);

        // 将处理后的内容写回文件
        let output_text = process_lines.join("\n");
        println!("output_text{}", output_text);
        std::fs::write(&file_path, output_text).unwrap_or_default();
        // 打开文件进行追加写入
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)
            .expect("无法打开文件");

        writeln!(file, "\n回放结果为：{}\n", playback_result).unwrap_or_default();
    }
}
