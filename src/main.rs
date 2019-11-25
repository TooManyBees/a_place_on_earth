mod twitter;
mod mastodon;

fn main() {
    let message = heaven::random_post();
    println!("{}", message);

    if let Err(_) = dotenv::dotenv() {
        eprintln!("No variables loaded from .env");
    }
    match mastodon::send(&message) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    match twitter::send(&message) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
