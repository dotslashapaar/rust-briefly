pub mod auth;

pub fn url_shortner(url: &str) -> String {
    let digest = md5::compute(url);
    let num = u128::from_be_bytes(digest.0);
    base62::encode(num).chars().take(7).collect()
}
