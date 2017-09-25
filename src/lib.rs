
pub trait Game {
    fn roll(&mut self, pins: i32);
    fn score(&self) -> i32;
}



