pub mod screen {

    use screenshots::Screen;

    //截图
    pub fn screenshot(b_x: f64, b_y: f64, x: f64, y: f64) {
        // 获取点所在屏幕
        let screen = Screen::from_point(100, 100).unwrap();
        // println!("点所在屏幕： {screen:?}");
        let width = x - b_x;
        let height = y - b_y;
        println!("x{},y{},width{},height{}", x, y, width, height);
        let image = screen
            .capture_area(
                b_x.floor() as i32,
                b_y.floor() as i32,
                width.floor() as u32,
                height.floor() as u32,
            )
            .unwrap();
        image
            .save("./result/capture_display_with_point.png")
            .unwrap();
    }
}
