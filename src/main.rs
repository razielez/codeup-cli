use std::env;

fn main() {
    let token = env::var("GITEE_TOKEN").unwrap();
    println!("token: {}", token)
}
