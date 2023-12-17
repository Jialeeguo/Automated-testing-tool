pub mod translate {
    use md5;
    use reqwest::Client;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TranslationResult {
        trans_result: Vec<Translation>,
    }

    #[derive(Deserialize)]                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              

    struct Translation {
        src: String,
        dst: String,
    }

    // async fn main() {
    //     let from_lang = String::from("en");
    //     let to_lang = String::from("zh");
    //     let query = String::from("Nankai university college of software");

    //     match translate(from_lang, to_lang, query).await {
    //         Ok((src, dst)) => {
    //             println!("Source: {}", src);
    //             println!("Translation: {}", dst);
    //         }
    //         Err(err) => {
    //             eprintln!("Error: {}", err);
    //         }
    //     }

    // }

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
