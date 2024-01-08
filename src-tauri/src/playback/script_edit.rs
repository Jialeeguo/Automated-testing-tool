pub mod script_edit {
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::io::{BufRead, BufReader};

    #[tauri::command]
    pub fn read_a_record(file_path:String) -> Vec<Vec<String>> {
        let file = File::open(format!("{}/record.txt",file_path)).expect("Error opening file");
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
                    //判断键盘content是什么内容（待优化）
                    //如果是单个字符，那就变成小写
                    if values[2].len() == 1 && values[2].chars().all(|c| c.is_ascii_uppercase()) {
                        let lowercase_char = values[2].chars().next().unwrap().to_ascii_lowercase();
                        row.push(lowercase_char.to_string());
                    } else {
                        //如果是其他按键，赋值
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
            // let mut row: Vec<String> = values.iter().map(|&s| s.to_string()).collect();

            script.push(row);
        }

        println!("{:?}", script);
        script
    }

    #[tauri::command]
    pub fn script_write_back(path:String,mut script: Vec<Vec<String>>) {
        script.remove(0);

        //判断是否为空
        if script.is_empty() {
            //处理为空
        }

        let file_path = format!("{}/record.txt",path);
        println!("要处理的二维数组{:?}", script);
        // let records_str: Vec<String> = script.iter().map(|row| row.join(",")).collect();

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

                    //判断键盘content是什么内容（待优化）
                    //如果是单个字符，那就变成小写
                    if line[2].len() == 1 && line[2].chars().all(|c| c.is_ascii_lowercase()) {
                        coordinate = line[2]
                            .chars()
                            .next()
                            .unwrap()
                            .to_ascii_uppercase()
                            .to_string();
                    } else {
                        //如果是其他按键，赋值
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
