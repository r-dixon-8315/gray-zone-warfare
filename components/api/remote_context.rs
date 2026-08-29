struct LiteClient {
    state: i64,
}

impl LiteClient {
    fn new(seed: i64) -> Self {
        LiteClient { state: seed }
    }

    fn fetch_builder(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 44) % 997;
        }
        result
    }
}

fn main() {
    let obj = LiteClient::new(44);
    println!("{}", obj.fetch_builder(44));
}
