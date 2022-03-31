extern crate parse_torrent;

use parse_torrent::Torrent;

fn main() {
    let test = Torrent::from_file("/mnt/usbhdd3/Okazaki/Torrent_Proxy/test/ubuntu-20.04.3-desktop-amd64.iso.torrent").unwrap();
    println!("test: {:?}", test);
}
