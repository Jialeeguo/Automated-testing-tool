// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{sync::Mutex, time::Instant};

mod playback;
mod record;
// use std::thread;
// use std::process;
// use std::time::Duration;
use playback::playback_main::playback_main::playback_confirm;
use playback::playback_main::playback_main::playback_main;
use playback::script_edit::script_edit::read_a_record;
use playback::script_edit::script_edit::script_write_back;
use record::record_main::record_main::pause_record;
use record::record_main::record_main::record_end;
use record::record_main::record_main::resume_record;
use record::record_main::record_main::start_record;
use record::record_main::record_main::start_screen;
use record::screen_shot::screen::screenshot;
use std::sync::{atomic::AtomicBool, Arc};
use tauri::Manager;
// use record::record_main::record_main::stop_record;
use playback::playback_main::playback_main::check_file_exists;
use playback::playback_main::playback_main::dir_confirm;
use playback::playback_main::playback_main::return_record_result;
use playback::playback_main::playback_main::search_test_dir;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
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
    //全局路径变量
    pub static ref NOW_DIR: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    //键盘是否停止监听标志
    pub static ref SHOULD_STOP_KEYBOARD: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    //暂停录制标志
    static ref PAUSE_FLAG: Mutex<bool> = Mutex::new(false);
    //暂停时间记录
    static ref PAUSE_TIME: Mutex<u128> = Mutex::new(0);
}
#[tauri::command]
//开屏界面
async fn close_splashscreen(window: tauri::Window) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    window.get_window("main").unwrap().show().unwrap();
}



fn a() {
    println!("1");
}
fn main() {
    let begin_transcribe = CustomMenuItem::new("transcribe".to_string(), "开始/终止录制(F1)");
    let screen_shot: CustomMenuItem = CustomMenuItem::new("screen_shot".to_string(), "截图(F2)");
    let end_transcribe = CustomMenuItem::new("end_transcribe".to_string(), "暂停/恢复录制(F4)");
    let running_transcribe: CustomMenuItem =
        CustomMenuItem::new("running_transcribe".to_string(), "运行脚本(F6)");
    let opening_script: CustomMenuItem =
        CustomMenuItem::new("opening_script".to_string(), "打开脚本(ctrl+O)");
    let save_script: CustomMenuItem =
        CustomMenuItem::new("save_script".to_string(), "保存脚本(ctrl+S)");
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
    let menu = Menu::new().add_submenu(script).add_submenu(running);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_record,
            playback_main,
            screenshot,
            start_screen,
            record_end,
            pause_record,
            resume_record,
            playback_confirm,
            close_splashscreen,
            read_a_record,
            script_write_back,
            check_file_exists,
            dir_confirm,
            return_record_result,
            search_test_dir//遍历指定文件夹下所有文件
        ])
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "transcribe" => {
                //没进方法，之后再改     
                a();
                start_record();
            }
            "screen_shot" => {
                // start_screen
                
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
