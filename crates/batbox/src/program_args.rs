use super::*;

pub fn args() -> Vec<String> {
    #[cfg(target_arch = "wasm32")]
    return {
        let mut args = vec!["program".to_owned()]; // `Program` itself is the first arg
        let url = url::Url::parse(&web_sys::window().unwrap().location().href().unwrap())
            .expect("Failed to parse window.location.href");
        for (key, value) in url.query_pairs() {
            let key: &str = &key;
            let value: &str = &value;
            if value.is_empty() {
                args.push("--".to_owned() + key);
            } else {
                args.push("--".to_owned() + key + "=" + value);
            }
        }
        trace!("href => args: {:?}", args);
        args
    };
    #[cfg(not(target_arch = "wasm32"))]
    return std::env::args().collect();
}

pub fn parse<T: clap::Parser>() -> T {
    clap::Parser::parse_from(args())
}
