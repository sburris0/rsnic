use crate::config::Config;
use crate::types::Content;
use serde_json::from_str;
use std::error::Error;

/// Returns the Content items found by searching for `query`
pub fn search(cfg: &Config, query: &str) -> Result<Vec<Content>, Box<dyn Error>> {
    let url = format!("{}/api/v1/search/?q={}&type=all", cfg.instance, query);
    let response: String = ureq::get(&url)
        .set("User-Agent", "Mozilla/5.0")
        .call()?
        .into_string()?;

    let deserialized: Vec<Content> = from_str(&response)?;
    Ok(deserialized[..cfg.results as usize].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        // Only get one result
        let mut cfg = Config::default();
        cfg.results = 1;

        let results: Vec<Content> = search(&cfg, "Max Stirner Complete Works Audio Book").unwrap();
        let top_result = results.first().unwrap();

        match top_result {
            Content::Video {
                video_id, title, ..
            } => {
                assert_eq!(video_id, "MmdB8R9sm4Y");
                assert_eq!(title, "Max Stirner Complete Works Audio Book");
            }
            _ => (),
        }
    }
}
