mod mastodon;
mod twitter;
use futures::{executor, future};

fn main() {
    let message = heaven::random_post();
    println!("Should we post this? (y/n)\n\n{}\n", message);

    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if !input.to_lowercase().starts_with('y') {
            return;
        }
    } else {
        return;
    }

    if let Err(_) = dotenv::dotenv() {
        eprintln!("No variables loaded from .env");
    }

    let (tweet, toot) = executor::block_on(future::join(
        twitter::send(&message),
        mastodon::send(&message),
    ));

    match tweet {
        Ok(_) => println!("Sent to twitter."),
        Err(e) => eprintln!("{}", e),
    }
    match toot {
        Ok(_) => println!("Sent to botsin.space."),
        Err(e) => eprintln!("{}", e),
    }
}
