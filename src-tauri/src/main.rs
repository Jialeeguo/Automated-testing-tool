// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{sync::Mutex, time::Instant};

mod playback;
mod record;
// use std::thread;
// use std::process;
// use std::time::Duration;
use playback::playback_main::playback_main::playback_main;
use record::record_main::record_main::start_record;
use record::screen_shot::screen::screenshot;
use record::record_main::record_main::start_screen;
use std::sync::Arc;
// use record::record_main::record_main::stop_record;
#[macro_use]
extern crate lazy_static;
//定义静态全局变量
lazy_static! {
    //计时器
    static ref START_TIME: Mutex<Instant> = Mutex::new(Instant::now());
    //鼠标最后移动时间
    static ref MOUSE_MOVE_TIME: Mutex<Option<u128>> = Mutex::new(None);
    //鼠标最后移动坐标
    static ref MOUSE_BEFORE_PRESS: Mutex<(f64,f64)> = Mutex::new((0.0,0.0));
    //判断是否启动截图flag
    static ref SCREEN_SHOT_FLAG: Mutex<bool> = Mutex::new(false);
    //截图起始坐标
    static ref SCREEN_PRESS: Mutex<(f64,f64)> = Mutex::new((0.0,0.0));
    //回放时最后动作时间
    static ref LAST_ACTION_TIME: Mutex<u128> = Mutex::new(0);
    //鼠标线程监听标志
    static ref MOUSE_THREAD_FLAG: Mutex<bool> = Mutex::new(false);
    //截图时间记录
    static ref TEXTSHOT_ACTION_TIME: Mutex<u128> = Mutex::new(0);
    // 是否停止录制的标志
    static ref SHOULD_STOP: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    //鼠标线程是否开启的标志
    static ref MOUSE_THREAD_START: Mutex<bool> = Mutex::new(false);
    //鼠标记录文件夹的位置
    static ref MOUSE_PATH: Mutex<String> = Mutex::new(String::new());

}
// #[tauri::command]
// pub fn stop_record() {
//     process::exit(0);
// }

fn main() {
    tauri::Builder::default()  
        .invoke_handler(tauri::generate_handler![
            start_record,
            playback_main,
            screenshot,
            start_screen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn main() {
//     loop {
//         println!("菜单：");
//         println!("1. 记录");
//         println!("2. 播放");
//         println!("3. 退出");

//         let choice = read_user_input("请输入选项：");

//         match choice.trim() {
//             "1" => record::record_main::record_main::start_record(),
//             "2" => playback::playback_main::playback_main::playback_main(),
//             "3" => break,
//             _ => println!("无效的选项"),
//         };
//     }
// }
