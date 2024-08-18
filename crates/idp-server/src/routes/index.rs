pub struct IndexHandler;

impl IndexHandler {
    pub async fn get() -> &'static str {
        "你好世界"
    }
}
