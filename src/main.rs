mod shapes {
    pub mod water;
    pub mod dock;
    pub mod fish;
    pub mod fisherman;
}
mod fishies;
use fishies::Fishies;

fn main() -> std::io::Result<()> {
    Fishies::run()
}
