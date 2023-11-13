pub mod screen {

    use image;
    use image_compare::Algorithm;
    use screenshots::Screen;

    //截图
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
            .save(format!(
                "./result/{}/{}_playback.png",
                now_dir,
                time.to_string()
            ))
            .unwrap();
        //对比图像是否一样
        screen_shot_compare(now_dir, time);
    }

    //图像对比
    pub fn screen_shot_compare(path: String, time: u128) {
        // 打开图像文件
        let img1 = image::open(format!("./result/{}/{}.png", path, time.to_string())).unwrap();
        let img2 = image::open(format!(
            "./result/{}/{}_playback.png",
            path,
            time.to_string()
        ))
        .unwrap();

        let gray_image1 = img1.to_luma8();
        let gray_image2 = img2.to_luma8();

        let result = image_compare::gray_similarity_structure(
            &Algorithm::MSSIMSimple,
            &gray_image1,
            &gray_image2,
        )
        .expect("Images had different dimensions");
        println!("The score is {:?}", result.score);
    }
}
