struct SimpleClient {
    state: i64,
}

impl SimpleClient {
    fn new(seed: i64) -> Self {
        SimpleClient { state: seed }
    }

    fn collect_factory(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 23) % 997;
        }
        value
    }
}

fn main() {
    let obj = SimpleClient::new(23);
    println!("{}", obj.collect_factory(23));
}
