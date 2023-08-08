use rolling_ohlc::RollingOHLC;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    // Open the input file (a.txt) and read the JSON data line by line
    let input_file = File::open("a.txt").expect("Failed to open the input file (a.txt).");
    let reader = BufReader::new(input_file);

    // Create the rolling OHLC instance with a window size of 5 minutes (in milliseconds)
    let window_size = 5 * 60 * 1000;
    let mut companies = Vec::new();
    companies.push("TURBOUSDT");
    companies.push("FISHUSDT");
    // Create a HashMap to store RollingOHLC instances for each company
    let mut company_objs: HashMap<String, RollingOHLC> = HashMap::new();
    // company_objs.insert("TURBOUSDT".to_string(), RollingOHLC::new(window_size));
    // company_objs.insert("FISHUSDT".to_string(), RollingOHLC::new(window_size));
    for i in 0..companies.len(){
    company_objs.insert(companies[i].to_string(), RollingOHLC::new(window_size));
    }

    // Create the output file (b.txt) and open it for writing
    let mut output_file = File::create("b.txt").expect("Failed to create the output file (b.txt).");

    // Process each line of JSON data from the input file
    for line in reader.lines() {
        if let Ok(json_line) = line {
            // Deserialize JSON data into PriceData struct
            let json_data: serde_json::Value =
                serde_json::from_str(&json_line).expect("Failed to parse JSON data.");

            // Extract the bid and ask prices from the JSON data
            let bid_price = json_data["b"]
                .as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or_default();
            let ask_price = json_data["a"]
                .as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or_default();

            // Calculate the ticker price as the average of bid and ask prices
            let ticker_price = (bid_price + ask_price) / 2.0;

            // Extract timestamp from JSON data
            let symbol = json_data["s"].as_str().unwrap_or_default().to_string();
            let timestamp = json_data["T"].as_u64().unwrap_or_default();

            // Find the corresponding RollingOHLC instance for the company
            if let Some(rolling_ohlc) = company_objs.get_mut(&symbol) {
                // Add the ticker price data to the rolling OHLC calculator
                rolling_ohlc.add_price(timestamp, ticker_price);

                // Calculate rolling OHLC over the last 5 minutes
                if let Some((open, high, low, close)) = rolling_ohlc.get_ohlc() {
                    // Prepare the output data in the desired format
                    let output_data = format!(
                        r#"{{"symbol":"{}","timestamp":{},"open":"{:.6}","high":"{:.6}","low":"{:.6}","close":"{:.6}"}}"#,
                        symbol, timestamp, open, high, low, close
                    );

                    // Write the output data to the output file (b.txt)
                    writeln!(output_file, "{}", output_data)
                        .expect("Failed to write to the output file (b.txt).");
                } else {
                    println!(
                        "Not enough data for rolling OHLC at timestamp: {}",
                        timestamp
                    );
                }
            }
        }
    }

    println!("Rolling OHLC calculations completed. Results written to b.txt.");
}
