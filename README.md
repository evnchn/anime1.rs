# anime1.rs
UNFINISHED CLI access to anime1.me (does not work)

## In the meantime

Check out anime1.py, https://github.com/evnchn/anime1.py

It actually works ~~and coding is less of a nightmare~~!

## TODOS
1. Parse res_text as JSON
2. get s.src
3. Parse all cookie_str instances
4. export to cookies.txt, Netscape format
5. get all cookies as header format
6. Call mpv

1, 2 and 6 seem doable, but there are no crates nor code snippets for handling 3, 4 and 5. 

## Rust and Python comparisons

### Parsing HTML

Python:

```python
try:
  page = requests.get(url)
  soup = bs.BeautifulSoup(page.text, 'lxml')
  json_content = requests.utils.unquote(soup.select("article")[0].select("video")[0]["data-apireq"])
except:
  print("unwrap?")
```

Rust: 

```rust
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
```

### Making HTTP request with appropriate headers and request data

Source: https://curlconverter.com/

Python: 

```python
headers = {
    'authority': 'v.anime1.me',
    'accept': '*/*',
    'accept-language': 'en-US,en;q=0.9,zh-TW;q=0.8,zh;q=0.7',
    'cache-control': 'no-cache',
    'dnt': '1',
    'origin': 'https://anime1.me',
    'pragma': 'no-cache',
    'referer': 'https://anime1.me/',
    'user-agent': 'MY_USER_AGENT',
}

data = {
    'd': '{"c":"1173","e":"2b","t":1673660514,"p":0,"s":"6c17f3b8efac3a29fcf42663b22ec57a"}',
}

response = requests.post('https://v.anime1.me/api', headers=headers, data=data)
```

Rust:

```rust
let mut headers = header::HeaderMap::new();
headers.insert("authority", "v.anime1.me".parse().unwrap());
headers.insert("accept", "*/*".parse().unwrap());
headers.insert("accept-language", "en-US,en;q=0.9,zh-TW;q=0.8,zh;q=0.7".parse().unwrap());
headers.insert("cache-control", "no-cache".parse().unwrap());
headers.insert("content-type", "application/x-www-form-urlencoded".parse().unwrap());
headers.insert("dnt", "1".parse().unwrap());
headers.insert("origin", "https://anime1.me".parse().unwrap());
headers.insert("pragma", "no-cache".parse().unwrap());
headers.insert("referer", "https://anime1.me/".parse().unwrap());
headers.insert("user-agent", "MY_USER_AGENT".parse().unwrap());

let client = reqwest::blocking::Client::builder()
	.redirect(reqwest::redirect::Policy::none())
	.build()
	.unwrap();
let res = client.post("https://v.anime1.me/api")
	.headers(headers)
	.body("d=%7B%22c%22%3A%221173%22%2C%22e%22%3A%222b%22%2C%22t%22%3A1673660514%2C%22p%22%3A0%2C%22s%22%3A%226c17f3b8efac3a29fcf42663b22ec57a%22%7D")
	.send()?
	.text()?;
println!("{}", res);
```
