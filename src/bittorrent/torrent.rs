use std::fs::File;
use std::io::Read;
use parse_torrent;


fn load_from_torrent(torrent_file_path: String) -> {
    torrent = parse_torrent::Torrent::from_file(&*torrent_file_path)
}