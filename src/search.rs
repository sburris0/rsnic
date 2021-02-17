use crate::config::Config;
use crate::types::Video;
use serde_json::from_str;
use std::error::Error;

/// Returns the videos found by searching for `query`
pub fn search(cfg: &Config, query: &str) -> Result<Vec<Video>, Box<dyn Error>> {
    let url = format!("{}/api/v1/search?q={}", cfg.instance, query);
    let search_result_string: String = ureq::get(&url)
        .set("User-Agent", "Mozilla/5.0")
        .call()?
        .into_string()?;

    Ok(from_str(&search_result_string)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        // Only get one result
        let mut cfg = Config::default();
        cfg.results = 1;

        let results = search(&cfg, "Max Stirner Complete Works Audio Book").unwrap();
        let top_result = &results.first().unwrap();
        assert_eq!(top_result.video_id, "MmdB8R9sm4Y".to_string());
        assert_eq!(
            top_result.title,
            "Max Stirner Complete Works Audio Book".to_string()
        );
    }
}
