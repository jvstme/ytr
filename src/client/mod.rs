use url::Url;
use reqwest::Client;
use std::borrow::Borrow;

const BASE_URL: &str = "https://translate.yandex.net/api/v1.5/tr.json";


pub struct TranslateAPI {
    key: String,
    client: Client,
}

impl TranslateAPI {
    pub fn new(key: String) -> TranslateAPI {
        let client = Client::new();
        TranslateAPI {
            key,
            client,
        }
    }

    fn make_url<I, K, V>(&self, method: &str, params: I) -> Url
        where I: IntoIterator,
              I::Item: Borrow<(K, V)>,
              K: AsRef<str>,
              V: AsRef<str>,
    {
        let mut url = Url::parse(BASE_URL).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(method);
        url.query_pairs_mut()
            .append_pair("key", &self.key)
            .extend_pairs(params);
        url
    }
}

#[cfg(test)]
mod tests;
