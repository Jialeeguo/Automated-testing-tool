pub mod script_edit {
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::io::{BufRead, BufReader,};
    use::std::path::Path;

    #[tauri::command]
    pub fn read_a_record(file_path: String) -> Result<Vec<Vec<String>>, String> {
        let path = Path::new(&file_path).join("record.txt");

        // 尝试打开文件，如果失败则返回错误
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err(format!("文件不存在: {:?}", path.display())),
        };
        let reader = BufReader::new(file);
        let mut script: Vec<Vec<String>> = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Error reading line");
            let values: Vec<&str> = line.split(',').collect();
            let mut action_name = String::new();
            let mut coordinate_or_key = String::new();
            let mut row: Vec<String> = Vec::new();
            row.push(values[0].to_string());
            match values[1] {
                "move" => {
                    action_name = "鼠标移动".to_string();
                    row.push(action_name);
                    let x = values[2].parse::<f64>().unwrap();
                    let y = values[3].parse::<f64>().unwrap();
                    coordinate_or_key = format!("({},{})", x, y);
                    row.push(coordinate_or_key);
                }
                "press" => match values[2] {
                    "Left" => {
                        action_name = "鼠标左键按下".to_string();
                        row.push(action_name);
                        let x = values[3].parse::<f64>().unwrap();
                        let y = values[4].parse::<f64>().unwrap();
                        coordinate_or_key = format!("({},{})", x, y);
                        row.push(coordinate_or_key);
                    }
                    "Right" => {
                        action_name = "鼠标右键按下".to_string();
                        row.push(action_name);
                        let x = values[3].parse::<f64>().unwrap();
                        let y = values[4].parse::<f64>().unwrap();
                        coordinate_or_key = format!("({},{})", x, y);
                        row.push(coordinate_or_key);
                    }
                    "Middle" => {
                        action_name = "鼠标中键按下".to_string();
                        row.push(action_name);
                        let x = values[3].parse::<f64>().unwrap();
                        let y = values[4].parse::<f64>().unwrap();
                        coordinate_or_key = format!("({},{})", x, y);
                        row.push(coordinate_or_key);
                    }
                    _ => {}
                },
                "release" => match values[2] {
                    "Left" => {
                        action_name = "鼠标左键释放".to_string();
                        row.push(action_name);
                    }
                    "Right" => {
                        action_name = "鼠标右键释放".to_string();
                        row.push(action_name);
                    }
                    "Middle" => {
                        action_name = "鼠标中键释放".to_string();
                        row.push(action_name);
                    }
                    _ => {}
                },
                "key" => {
                    action_name = "键盘按键".to_string();
                    row.push(action_name);
                    if values[2].len() == 1 && values[2].chars().all(|c| c.is_ascii_uppercase()) {
                        let lowercase_char = values[2].chars().next().unwrap().to_ascii_lowercase();
                        row.push(lowercase_char.to_string());
                    } else {
                        coordinate_or_key = values[2].to_string();
                        row.push(coordinate_or_key);
                    }
                }
                "screen_press" => match values[2] {
                    "Left" => {
                        action_name = "截图开始".to_string();
                        row.push(action_name);
                        let x = values[3].parse::<f64>().unwrap();
                        let y = values[4].parse::<f64>().unwrap();
                        coordinate_or_key = format!("({},{})", x, y);
                        row.push(coordinate_or_key);
                    }
                    _ => {}
                },
                "screen_release" => match values[2] {
                    "Left" => {
                        action_name = "截图结束".to_string();
                        row.push(action_name);
                        let x = values[3].parse::<f64>().unwrap();
                        let y = values[4].parse::<f64>().unwrap();
                        coordinate_or_key = format!("({},{})", x, y);
                        row.push(coordinate_or_key);
                    }
                    _ => {}
                },
                _ => {}
            }
            script.push(row);
        }

        println!("{:?}", script);
        
    Ok(script)
    }

    #[tauri::command]
    pub fn script_write_back(path:String,mut script: Vec<Vec<String>>) {
        script.remove(0);

        if script.is_empty() {
        }

        let file_path = format!("{}/record.txt",path);
        println!("要处理的二维数组{:?}", script);
        let mut file = OpenOptions::new().write(true).open(file_path).unwrap();
        file.set_len(0).unwrap();

        for line in script {
            let mut values: Vec<String> = Vec::new();

            match line[1].as_str() {
                "鼠标移动" => {
                    let coordinate = line[2].trim_matches(|c| c == '(' || c == ')');
                    let action = format!("{},move,{}\n", line[0], coordinate.to_string());
                    file.write_all(action.as_bytes()).unwrap();
                }
                "鼠标左键按下" => {
                    let coordinate = line[2].trim_matches(|c| c == '(' || c == ')');
                    let action = format!("{},press,Left,{}\n", line[0], coordinate.to_string());
                    file.write_all(action.as_bytes()).unwrap();
                }
                "鼠标右键按下" => {
                    let coordinate = line[2].trim_matches(|c| c == '(' || c == ')');
                    let action = format!("{},press,Right,{}\n", line[0], coordinate.to_string());
                    file.write_all(action.as_bytes()).unwrap();
                }
                "鼠标中键按下" => {
                    let coordinate = line[2].trim_matches(|c| c == '(' || c == ')');
                    let action = format!("{},press,Middle,{}\n", line[0], coordinate.to_string());
                    file.write_all(action.as_bytes()).unwrap();
                }
                "鼠标左键释放" => {
                    let action = format!("{},release,Left\n", line[0]);
                    file.write_all(action.as_bytes()).unwrap();
                }
                "鼠标右键释放" => {
                    let action = format!("{},release,Right\n", line[0]);
                    file.write_all(action.as_bytes()).unwrap();
                }
                "鼠标中键释放" => {
                    let action = format!("{},release,Middle\n", line[0]);
                    file.write_all(action.as_bytes()).unwrap();
                }
                "键盘按键" => {
                    let mut coordinate = String::new();

                    if line[2].len() == 1 && line[2].chars().all(|c| c.is_ascii_lowercase()) {
                        coordinate = line[2]
                            .chars()
                            .next()
                            .unwrap()
                            .to_ascii_uppercase()
                            .to_string();
                    } else {
                        coordinate = line[2].clone();
                    }
                    let action = format!("{},key,{}\n", line[0], coordinate);
                    file.write_all(action.as_bytes()).unwrap();
                }
                "截图开始" => {
                    let coordinate = line[2].trim_matches(|c| c == '(' || c == ')');
                    let action =
                        format!("{},screen_press,Left,{}\n", line[0], coordinate.to_string());
                    file.write_all(action.as_bytes()).unwrap();
                }
                "截图结束" => {
                    let coordinate = line[2].trim_matches(|c| c == '(' || c == ')');
                    let action = format!(
                        "{},screen_release,Left,{}\n",
                        line[0],
                        coordinate.to_string()
                    );
                    file.write_all(action.as_bytes()).unwrap();
                }
                _ => {}
            }
        }
    }
}
