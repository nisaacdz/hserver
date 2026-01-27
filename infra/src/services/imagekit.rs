use app::settings::ImageKitSettings;

pub fn generate_url(external_id: &str, config: &ImageKitSettings) -> String {
    let mut url = config.url.clone();
    if let Ok(mut segments) = url.path_segments_mut() {
        segments.push(external_id);
    }
    url.to_string()
}
