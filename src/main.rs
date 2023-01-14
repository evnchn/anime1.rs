use urlencoding::decode;

// use std::error::Error;
// use reqwest::header::SetCookie;

extern crate reqwest;
use reqwest::header;

use reqwest::header::SET_COOKIE;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_state = "https://anime1.me/17173".to_string();

    if current_state.contains("://") {
        // println!("{}", current_state.split("://").nth(1).unwrap());
        if current_state
            .split("://")
            .nth(1)
            .unwrap()
            .replace("anime1.me/", "")
            .bytes()
            .all(|c| c.is_ascii_digit())
        {
            let o = minreq::get(current_state).send()?;
            let s = o.as_str()?;

            let dom = tl::parse(s, tl::ParserOptions::default()).unwrap();
            let parser = dom.parser();
            let attributes = dom
                .query_selector("video")
                .unwrap()
                .nth(0)
                .unwrap()
                .get(parser)
                .unwrap()
                .as_tag()
                .unwrap()
                .attributes();
            let crux = attributes
                .get("data-apireq")
                .unwrap()
                .unwrap()
                .as_utf8_str();
            let decoded = decode(&crux).expect("UTF-8");
            println!("{}", decoded);

            let mut headers = header::HeaderMap::new();
            headers.insert("authority", "v.anime1.me".parse().unwrap());
            headers.insert("accept", "*/*".parse().unwrap());
            headers.insert(
                "accept-language",
                "en-US,en;q=0.9,zh-TW;q=0.8,zh;q=0.7".parse().unwrap(),
            );
            headers.insert("cache-control", "no-cache".parse().unwrap());
            headers.insert(
                "content-type",
                "application/x-www-form-urlencoded".parse().unwrap(),
            );
            headers.insert("dnt", "1".parse().unwrap());
            headers.insert("origin", "https://anime1.me".parse().unwrap());
            headers.insert("pragma", "no-cache".parse().unwrap());
            headers.insert("referer", "https://anime1.me/".parse().unwrap());
            headers.insert("user-agent", "Mozilla/5.0 (Linux; Android 9; AMN-LX9 Build/HUAWEIAMN-LX9; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/97.0.4692.70 Mobile Safari/537.36 [FB_IAB/FB4A;FBAV/348.0.0.39.118;]".parse().unwrap());

            let client = reqwest::blocking::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .cookie_store(true)
                .build()
                .unwrap();

            let outsr = "d=".to_string() + &crux;
            println!("{}", outsr);

            let res = client
                .post("https://v.anime1.me/api")
                .headers(headers)
                .body("d=".to_string() + &crux)
                .send()?;

            let set_cookie_iter = res.headers().get_all(SET_COOKIE);

            for c in set_cookie_iter {
                c.to_str()
                    .into_iter()
                    .map(|s| s.to_string())
                    .for_each(|cookie_str| {
                        // Cookie::parse(cookie_str).into_iter().for_each(|cookie| {
                        //     self.jar.add_original(cookie);
                        // });
                        println!("{}", cookie_str);
                    });
            }

            let res_text = res.text()?;
            println!("{}", res_text);

            /*

            TODOS:
            1. Parse res_text as JSON
            2. get s.src
            3. Parse all cookie_str instances
            4. export to cookies.txt, Netscape format
            5. get all cookies as header format
            6. Call mpv

            */

            //println!("{}", cookies);
            Ok(())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
