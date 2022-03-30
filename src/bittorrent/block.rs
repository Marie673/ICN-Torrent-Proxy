const BLOCK_SIZE: u32 = 2 ** 14;

enum State {
    FREE,
    PENDING,
    FULL
}

struct Block {
    state: State,
    block_size: u32,
    data: [u8; block_size],
    last_seen: i64
}