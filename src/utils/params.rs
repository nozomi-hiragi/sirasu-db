use std::env;

fn get_value_or_default(env_key: &str, default_value: &str) -> String {
    match env::var(env_key) {
        Ok(v) => v,
        Err(_) => String::from(default_value),
    }
}

pub struct Params {
    pub port: String,
    pub mongodb_url: String,
}

impl Params {
    pub fn new() -> Self {
        let mut port = get_value_or_default("PORT", "8080");
        let mut mongodb_url = get_value_or_default("MONGODB", "mongodb://localhost:27017");

        let args: Vec<String> = env::args().collect();
        let len = args.len();
        let mut idx = 0;
        while len > idx {
            let arg = &*args[idx];
            match arg {
                "-p" => {
                    idx += 1;
                    if idx >= len {
                        continue;
                    }
                    port = args[idx].clone();
                }
                "--mongodb" => {
                    idx += 1;
                    if idx >= len {
                        continue;
                    }
                    mongodb_url = args[idx].clone();
                }
                _ => {}
            }
            idx += 1;
        }

        Params { port, mongodb_url }
    }
}
