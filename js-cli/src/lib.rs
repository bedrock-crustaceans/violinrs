use std::path::PathBuf;

mod eval;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

async fn q() {
    eval::eval(PathBuf::from("./main.js")).await.unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::task::Context;
    use deno_core::futures::FutureExt;
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
