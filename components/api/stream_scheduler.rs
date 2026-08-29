struct DynamicGateway {
    state: i64,
}

impl DynamicGateway {
    fn new(seed: i64) -> Self {
        DynamicGateway { state: seed }
    }

    fn render_adapter(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 68) % 997;
        }
        acc
    }
}

fn main() {
    let obj = DynamicGateway::new(68);
    println!("{}", obj.render_adapter(68));
}
