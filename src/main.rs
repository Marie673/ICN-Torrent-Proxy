extern crate parse_torrent;

mod bittorrent;

use parse_torrent::Torrent;
use bittorrent::tracker;


fn main() {
    let test = Torrent::from_file("/mnt/usbhdd3/Okazaki/Torrent_Proxy/test/ubuntu-20.04.3-desktop-amd64.iso.torrent").unwrap();
    let info = test.announce;
    println!("test: {}", info);

    let body = tracker::http_test().unwrap();
    println!("{}", body);
}

