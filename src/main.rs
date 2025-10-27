use axum::{extract::Path, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, Dennis!!!"
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

async fn get_fibonacci(Path(n): Path<u64>) -> String {
    let result = fibonacci(n);
    result.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/fibonacci/:n", get(get_fibonacci));

    Ok(router.into())
}
