use app::settings::ImageKitSettings;
use url::Url;

pub fn generate_url(external_id: &str, config: &ImageKitSettings) -> String {
    let base_url = if config.url.ends_with('/') {
        config.url.clone()
    } else {
        format!("{}/", config.url)
    };

    match Url::parse(&base_url) {
        Ok(mut url) => {
            if let Ok(mut segments) = url.path_segments_mut() {
                segments.push(external_id);
            }
            url.to_string()
        }
        Err(_) => {
            // Fallback for invalid base URL configuration, though this should be caught earlier
            format!("{}{}", base_url, external_id)
        }
    }
}
