mod shapes {
    pub mod water;
    pub mod dock;
    pub mod fish;
}
mod fishies;
use fishies::Fishies;

fn main() -> std::io::Result<()> {
    Fishies::run()
}
