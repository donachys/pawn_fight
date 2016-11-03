pub trait Player {
    fn get_moves(&self) -> Option<((i32, i32), (i32, i32))>;
}