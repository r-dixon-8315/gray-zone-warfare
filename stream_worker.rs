struct LiteProvider {
    state: i64,
}

impl LiteProvider {
    fn new(seed: i64) -> Self {
        LiteProvider { state: seed }
    }

    fn resolve_session(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 31) % 997;
        }
        result
    }
}

fn main() {
    let obj = LiteProvider::new(31);
    println!("{}", obj.resolve_session(31));
}
