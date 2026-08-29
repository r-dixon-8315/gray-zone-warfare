struct LocalEngine {
    state: i64,
}

impl LocalEngine {
    fn new(seed: i64) -> Self {
        LocalEngine { state: seed }
    }

    fn parse_router(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 37) % 997;
        }
        result
    }
}

fn main() {
    let obj = LocalEngine::new(37);
    println!("{}", obj.parse_router(37));
}
