// todo
// use logs::{error, info, warn};


#[cfg(test)]
mod tests {
    #[test]
    fn lookup() {
        use log::warn;
        use log::info;
        use std::env;

        let _ = env_logger::builder().is_test(true).try_init();

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
        info!("get_answer: {} {:?}", domain, query);
        // warn!("get_answer: {} {:?}", domain, query);
    }
}
