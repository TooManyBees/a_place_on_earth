mod mastodon;
mod twitter;
use futures::{executor, future};

fn main() {
    let message = heaven::random_post();
    println!("{}", message);

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
