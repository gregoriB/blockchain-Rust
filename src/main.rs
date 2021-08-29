mod blockchain;
mod block;
mod transaction;
mod common;

use blockchain::BlockChain;

fn main() {
    let my_coin = BlockChain::new("My Coin", 3, 10, 1000000);
    println!("{:?}", my_coin.get_data());
}