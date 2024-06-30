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
    use std::path::PathBuf;
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
        // 检查file_path是否为空
        if file_path.is_empty() {
            println!("file_path为空");
            return; // 如果为空则输出提示并退出函数
        }
        println!("path:{}", file_path);
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
                "<!DOCTYPE html>
<html lang=\"zh-CN\">
<head>
    <meta charset=\"UTF-8\">
    <title>{}脚本回放日志 时间：{}</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: #6A006A;
            color: white;
            margin: 0;
            padding: 0;
        }}
        .container {{
            display: flex;
            justify-content: space-around;
            padding: 20px;
        }}
        .section {{
            background-color: #9B009B;
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
            width: 30%;
        }}
        .section h2 {{
            text-align: center;
            margin-bottom: 20px;
        }}
        .chart {{
            height: 200px;
        }}
        .table-container {{
            background-color: #B300B3;
            border-radius: 10px;
            padding: 10px;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        table, th, td {{
            border: 1px solid white;
        }}
        th, td {{
            padding: 10px;
            text-align: center;
        }}
        .donut-chart {{
            position: relative;
            width: 200px;
            height: 200px;
            border-radius: 50%;
            background: conic-gradient(#00FF00 0% 63.33%, #FFFF00 63.33% 90%, #FF0000 90% 100%);
            margin: 0 auto;
        }}
        .donut-chart::before {{
            content: '';
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            width: 100px;
            height: 100px;
            background-color: #6A006A;
            border-radius: 50%;
        }}
        .donut-chart p {{
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            margin: 0;
            font-size: 20px;
        }}
        .legend {{
            display: flex;
            justify-content: center;
            margin-top: 10px;
        }}
        .legend div {{
            display: flex;
            align-items: center;
            margin: 0 10px;
        }}
        .legend div span {{
            display: inline-block;
            width: 20px;
            height: 20px;
            margin-right: 5px;
        }}
        .legend .pass {{
            background-color: #00FF00;
        }}
        .legend .fail {{
            background-color: #FFFF00;
        }}
        .legend .pending {{
            background-color: #FF0000;
        }}
    </style>
</head>
<body>
    <h1 style=\"text-align: center;\">自动化测试报告</h1>
    <div class=\"container\">
        <div class=\"section\">
            <h2>执行结果</h2>
            <div>
                <p>通过用例: 19条</p>
                <p>失败用例: 12条</p>
                <p>暂定用例: 4条</p>
            </div>
            <h2>成功占比</h2>
            <div class=\"donut-chart\">
                <p>通过</p>
            </div>
            <div class=\"legend\">
                <div><span class=\"pass\"></span>通过</div>
                <div><span class=\"fail\"></span>失败</div>
                <div><span class=\"pending\"></span>暂定</div>
            </div>
        </div>
        <div class=\"section\">
            <h2>运行信息</h2>
            <div>
                <p>开始时间: {}</p>
                <p>用例总数: 30</p>
                <p>测试人员: 用旭涵</p>
                <p>成功用例: 30</p>
                <p>通过率: 100%</p>
                <p>运行时间: 20.05S</p>
            </div>
        </div>
        <div class=\"section\">
            <h2>历史构建结果</h2>
            <div class=\"table-container\">
                <table>
                    <tr>
                        <th>执行结果</th>
                        <th>用例总数</th>
                        <th>成功用例</th>
                        <th>通过率</th>
                    </tr>
                    <tr>
                        <td>2024-01-12 18:08:31</td>
                        <td>30</td>
                        <td>30</td>
                        <td>100%</td>
                    </tr>
                    <tr>
                        <td>2024-01-12 18:08:31</td>
                        <td>18</td>
                        <td>12</td>
                        <td>66.66%</td>
                    </tr>
                    <tr>
                        <td>2024-01-12 18:08:31</td>
                        <td>28</td>
                        <td>26</td>
                        <td>92.85%</td>
                    </tr>
                    <tr>
                        <td>2024-01-12 18:08:31</td>
                        <td>15</td>
                        <td>10</td>
                        <td>66.67%</td>
                    </tr>
                    <tr>
                        <td>2024-01-12 18:08:31</td>
                        <td>7</td>
                        <td>4</td>
                        <td>57%</td>
                    </tr>
                </table>
            </div>
        </div>
    </div>",
                now_dir, back_time, back_time
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

    //回放多个脚本
    // #[tauri::command]
    // pub fn batch_select(path:String,selected: Vec<String>,language:String){
    //     // script.remove(0);

    //     if selected.is_empty() {
    //         //前端可以提前判断一下 如果没内容log输出一下
    //     }
    //     println!("script:{:?}",selected);
    //     println!("path:{}",path);
    //     println!("language:{}",language);
    //     for script in selected{
    //         let file_path = format!("{}{}",path,script);
    //         // playback_main(file_path,language);
    //     }
    // }

    //批量运行时重命名
    #[tauri::command]
    pub fn rename_directory(old_name: String, new_name: String, path: String) -> Result<(), String> {
        let mut old_path = PathBuf::from(&path);
        old_path.push(&old_name);
    
        let mut new_path = PathBuf::from(&path);
        new_path.push(&new_name);
    
        if old_path.exists() {
            fs::rename(&old_path, &new_path)
                .map_err(|e| format!("Failed to rename directory: {}", e))?;
            Ok(())
        } else {
            Err("Old directory does not exist".into())
        }
    }

}
