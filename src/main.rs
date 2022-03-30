mod bittorrent;
use bittorrent::block;

fn main() {
    bittorrent::ping();
    block::echo();
}
