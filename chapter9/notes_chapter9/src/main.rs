fn main() {
    println!("Hello, world!");
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

/// `Broom`이 할 수 있는 일 두가지
#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}
