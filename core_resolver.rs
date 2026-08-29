struct CoreClient {
    state: i64,
}

impl CoreClient {
    fn new(seed: i64) -> Self {
        CoreClient { state: seed }
    }

    fn encode_loader(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 17) % 997;
        }
        total
    }
}

fn main() {
    let obj = CoreClient::new(17);
    println!("{}", obj.encode_loader(17));
}
