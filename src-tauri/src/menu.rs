// src/menu.rs

use tauri::{Menu, MenuItem, CustomMenuItem, Submenu};

pub fn get_main_menu() -> Menu {
    let begin_transcribe = CustomMenuItem::new("transcribe".to_string(), "开始/终止录制(F1)");
    let screen_shot = CustomMenuItem::new("screen_shot".to_string(), "截图(F2)");
    let end_transcribe = CustomMenuItem::new("end_transcribe".to_string(), "暂停/恢复录制(F4)");
    let running_transcribe = CustomMenuItem::new("running_transcribe".to_string(), "运行脚本(F6)");
    let opening_script = CustomMenuItem::new("opening_script".to_string(), "打开脚本(ctrl+O)");
    let save_script = CustomMenuItem::new("save_script".to_string(), "保存脚本(ctrl+S)");
    let auto = CustomMenuItem::new("auto".to_string(), "自动识别");
    let eng = CustomMenuItem::new("eng".to_string(), "英语");
    let spa = CustomMenuItem::new("spa".to_string(), "西班牙语");
    let ara = CustomMenuItem::new("ara".to_string(), "阿拉伯语");
    let por = CustomMenuItem::new("por".to_string(), "葡萄牙语");
    let rus = CustomMenuItem::new("rus".to_string(), "俄语");
    let fre = CustomMenuItem::new("fre".to_string(), "法语");
    let chi = CustomMenuItem::new("chi".to_string(), "中文");
    let script = Submenu::new(
        "文件",
        Menu::new().add_item(opening_script).add_item(save_script),
    );

    let running = Submenu::new(
        "运行",
        Menu::new()
            .add_item(begin_transcribe)
            .add_item(screen_shot)
            .add_item(end_transcribe)
            .add_item(running_transcribe),
    );

    let lang = Submenu::new(
        "翻译语种",
        Menu::new()
            .add_item(auto)
            .add_item(eng)
            .add_item(spa)
            .add_item(ara)
            .add_item(por)
            .add_item(rus)
            .add_item(fre)
            .add_item(chi)
            ,
    );

    Menu::new().add_submenu(script).add_submenu(running).add_submenu(lang)
}
