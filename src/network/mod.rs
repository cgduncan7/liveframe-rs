use lazy_static::lazy_static;
use url::Url;

lazy_static! {
  static ref CLIENT: reqwest::blocking::Client = {
    let client = reqwest::blocking::Client::new();
    client
  };
}

pub fn perform_request(method: reqwest::Method, url_str: &str) -> reqwest::Result<reqwest::blocking::Response> {
  let url: Url = match Url::parse(url_str) {
    Ok(url) => url,
    Err(_) => panic!("error parsing url for photo")
  };

  match method {
    reqwest::Method::GET => {
      CLIENT.get(url).send()
    },
    _ => panic!("unsupported method"),
  }
}
