use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let provider = yahoo::YahooConnector::new();
    let resp = tokio_test::block_on(provider.search_ticker("Apple")).unwrap();

    let mut apple_found = false;
    println!("All tickers found while searching for 'Apple':");
    for item in resp.quotes {
        println!("{}", item.symbol)
    }
}
