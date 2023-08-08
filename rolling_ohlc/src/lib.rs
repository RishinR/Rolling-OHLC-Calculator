#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PriceData{
    pub timestamp: u64,
    pub price: f64,
}

pub struct RollingOHLC {
    pub buffer: Vec<PriceData>,
    pub window_size: u64,
}
impl RollingOHLC {
    pub fn new(window_size: u64) -> Self {
        RollingOHLC {
            buffer: Vec::new(),
            window_size,
        }
    }

    pub fn add_price(&mut self, timestamp: u64, price: f64) {
        // Add the new price data to the buffer
        self.buffer.push(PriceData {
            timestamp,
            price,
        });

        // Remove old data outside the time window
        let time_window = timestamp - self.window_size;
        self.buffer.retain(|data| data.timestamp >= time_window);
    }

    pub fn get_ohlc(&self) -> Option<(f64, f64, f64, f64)> {
        // Calculate OHLC values over the current time window and return as a tuple (open, high, low, close)
        if self.buffer.is_empty() {
            return None;
        }

        let open = self.buffer[0].price;
        let mut high = self.buffer[0].price;
        let mut low = self.buffer[0].price;
        let mut close = self.buffer[0].price;

        for data in &self.buffer {
            if data.price > high {
                high = data.price;
            }
            if data.price < low {
                low = data.price;
            }
            close = data.price;
        }

        Some((open, high, low, close))
    }
}

