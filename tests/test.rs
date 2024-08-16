use logs::{info, warn};

fn main() {
    #[derive(Debug)]
    struct Query {
        keyword: String,
        page: u32,
    }

    let domain = "example.com";
    let query = Query {
        keyword: "rust".to_string(),
        page: 1,
    };

    info!("get_answer: {} {:?}", domain, query);
    warn!("get_answer: {} {:?}", domain, query);
}
