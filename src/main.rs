use std::io;

mod add;
mod complete;
mod delete;
mod list;
mod matching;

fn main() -> io::Result<()> {
    matching::matching()
}
