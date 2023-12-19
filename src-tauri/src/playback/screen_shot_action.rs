pub mod screen {
    use image;
    use image_compare::Algorithm;
    use md5;
    use reqwest::Client;
    use screenshots::Screen;
    use serde::Deserialize;
    use std::process::Command;
    use std::{fs::OpenOptions, io::Write};
    use tokio;
    #[derive(Deserialize)]
    struct TranslationResult {
        trans_result: Vec<Translation>,
    }

    #[derive(Deserialize)]

    struct Translation {
        src: String,
        dst: String,
    }

    //截图
    pub fn screenshot(
        b_x: f64,
        b_y: f64,
        x: f64,
        y: f64,
        time: u128,
        now_dir: String,
        lang: String,
    ) {
        // 获取点所在屏幕
        // let mut log_path = String::from(file_path);
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
            .save(format!("{}/{}_playback.png", now_dir, time.to_string()))
            .unwrap();
        //对比图像是否一样
        screen_shot_compare_and_text_compare(now_dir, time, lang);
    }

    //图像对比
    pub fn screen_shot_compare_and_text_compare(path: String, time: u128, lang: String) {
        // 打开图像文件
        let img1 = image::open(format!("{}/{}.png", path, time.to_string())).unwrap();
        let img2_path = format!("{}/{}_playback.png", path, time.to_string());
        let img2 = image::open(img2_path.clone()).unwrap();

        Command::new("tesseract")
            .current_dir(format!("{}", path))
            .arg(format!("{}_playback.png", time.to_string()))
            .arg(format!("textshot_{}_playback", time.to_string()))
            .arg("-l")
            .arg("eng+chi_sim")
            .output()
            .expect("提取文字错误");

        //灰度值
        let gray_image1 = img1.to_luma8();
        let gray_image2 = img2.to_luma8();

        let result = image_compare::gray_similarity_structure(
            &Algorithm::MSSIMSimple,
            &gray_image1,
            &gray_image2,
        )
        .expect("Images had different dimensions");

        // 打开提取文字文件
        let text1 = std::fs::read_to_string(&format!("{}/textshot_{}.txt", path, time.to_string()))
            .expect("无法读取文件1的内容");

        let text2 = std::fs::read_to_string(&format!(
            "{}/textshot_{}_playback.txt",
            path,
            time.to_string()
        ))
        .expect("无法读取文件2的内容");

        if !text2.is_empty() {
            let modified_text2 = &text2[..text2.len() - 1];
            std::fs::write(
                &format!("{}/textshot_{}_playback.txt", path, time.to_string()),
                modified_text2,
            )
            .expect("无法写入文件");
        }

        let text2_new = std::fs::read_to_string(&format!(
            "{}/textshot_{}_playback.txt",
            path,
            time.to_string()
        ))
        .expect("无法读取文件2的内容");

        let mut translation = String::new();

        if !text2_new.is_empty() && lang != "zh" {
            tokio::runtime::Runtime::new()
                .expect("Failed to create Tokio runtime")
                .block_on(async {
                    let (_src, dst) =
                        translate(lang, "zh".to_string(), text2_new.clone().replace("\n", " "))
                            .await
                            .map_err(|err| {
                                println!("Failed to get translation: {:?}", err);
                                "回放时没有查找到字体,请人工查看回放与录制时截图的操作是否一致（本条消息自动生成，非截图生成的字体）".to_string()
                            })
                            .unwrap_or_else(|default_value| ("回放时没有查找到字体,请人工查看回放与录制时截图的操作是否一致（本条消息自动生成，非截图生成的字体）".to_string(), default_value));
                    translation = dst;
                    
                });
        }

        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/record_result.txt", path))
            .unwrap();

        // 写入内容
        writeln!(save_file, "文字提取录制结果:\n{}", text1).expect("写入失败");
        if text2_new.is_empty(){
            writeln!(save_file, "文字提取回放结果:{}", text2_new).expect("写入失败");
            writeln!(save_file,"回放时没有查找到字体,请人工查看回放与录制时截图的操作是否一致（本条消息自动生成，非截图生成的字体）\n",).expect("写入失败");
        }else{
        writeln!(save_file, "文字提取回放结果:\n{}\n", text2_new).expect("写入失败");
    }
        if !translation.is_empty() {
            writeln!(save_file, "文字提取翻译结果:\n{}\n", translation).expect("翻译结果写入失败");
        }
        let time = "当前时间";
        if "text1" == "text2_new" {
            writeln!(save_file, "{}时刻文字提取对比验证通过！\n", time).expect("写入失败");
        } else if "text1" == "translation" {
            writeln!(save_file, "{}时刻文字提取对比验证通过！", time).expect("写入失败");
        } else {
            writeln!(save_file, "{}文字提取对比结果不相同！", time).expect("写入失败");
        }
        writeln!(
            save_file,
            "{}时刻对比图像相似度： {:?}",
            time.to_string(),
            result.score
        )
        .expect("写入失败");
  
    }

    pub async fn translate(
        from_lang: String,
        to_lang: String,
        query: String,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let client = Client::new();
        let appid = "20231209001905731";
        let appkey = "7aTtCh0dXiKXwFLcyO0n";
        // let from_lang = "en";
        // let to_lang = "zh";
        // let query = "Nankai university college of software";

        let salt = rand::random::<u32>();
        let sign_string = format!("{}{}{}{}", appid, query, salt, appkey);
        println!("{}", sign_string);

        let sign = md5::compute(sign_string);

        println!("md5:{:?}", sign);

        let url: String = format!(
        "http://api.fanyi.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={:?}",
        query, from_lang, to_lang, appid, salt, sign
    );
        println!("url:{}", url);

        let response = client.get(&url).send().await?;
        let result: TranslationResult = response.json().await?;

        let mut trans_source = String::new();
        let mut trans_translation = String::new();

        for trans in result.trans_result {
            trans_source = trans.src.clone();
            trans_translation = trans.dst.clone();
            println!("Source: {}", trans.src);
            println!("Translation: {}", trans.dst);
        }

        Ok((trans_source, trans_translation))
    }
}
