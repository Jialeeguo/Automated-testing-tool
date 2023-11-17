pub mod screen {

    use screenshots::Screen;
    use crate::TEXTSHOT_ACTION_TIME;

    //截图
    #[tauri::command]
    pub fn screenshot(b_x: f64, b_y: f64, x: f64, y: f64, time: u128, now_dir: String) {
        // 获取点所在屏幕
        let screen = Screen::from_point(100, 100).unwrap();
        // println!("点所在屏幕： {screen:?}");
        let width = x - b_x - 2.0;
        let height = y - b_y - 2.0;
        println!("x{},y{},width{},height{}", x, y, width, height);
        let image = screen
            .capture_area(
                (b_x + 2.0).floor() as i32,
                (b_y + 2.0).floor() as i32,
                width.floor() as u32,
                height.floor() as u32,
            )
            .unwrap();
        image
            .save(format!("../Automated-testing/result/{}/{}.png", now_dir, time.to_string()))
            .unwrap();
        *TEXTSHOT_ACTION_TIME.lock().unwrap() =time;
    }
}
