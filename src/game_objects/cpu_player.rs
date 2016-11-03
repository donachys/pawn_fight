use traits::Player;

pub struct CpuPlayer {
    selection: Option<(i32, i32)>,
    player_num: i32,
    move_buffer: Option<((i32, i32), (i32, i32))>
}

impl CpuPlayer {
    pub fn new(p: i32) -> CpuPlayer{
        CpuPlayer{
            selection: None,
            player_num: p,
            move_buffer: None
        }
    }
}

impl Player for CpuPlayer {
    fn get_moves(&self) -> Option<((i32, i32), (i32, i32))> { self.move_buffer }
}