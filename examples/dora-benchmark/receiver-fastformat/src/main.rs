use dora_node_api::{self, DoraNode, Event};
use std::time::{Duration, Instant};

use fastformat::datatypes::Image;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_node, mut events) = DoraNode::init_from_env()?;

    // latency is tested first
    let mut latency = true;

    let mut current_size = 0;
    let mut n = 0;
    let mut start = Instant::now();
    let mut latencies = Vec::new();

    println!("Latency:");

    while let Some(event) = events.recv() {
        match event {
            Event::Input { id, metadata, data } => {
                use arrow::array::Array;

                let image_raw = Image::raw_data(data.0.into_data())?;
                let image = Image::view_from_raw_data(&image_raw)?;

                // check if new size bracket
                let data_len = image.data.len();
                if data_len != current_size {
                    if n > 0 {
                        record_results(start, current_size, n, latencies, latency);
                    }
                    current_size = data_len;
                    n = 0;
                    start = Instant::now();
                    latencies = Vec::new();
                }

                match id.as_str() {
                    "latency" if latency => {}
                    "throughput" if latency => {
                        latency = false;
                        println!("Throughput:");
                    }
                    "throughput" => {}
                    other => {
                        eprintln!("Ignoring unexpected input `{other}`");
                        continue;
                    }
                }

                n += 1;
                latencies.push(
                    metadata
                        .timestamp()
                        .get_time()
                        .to_system_time()
                        .elapsed()
                        .unwrap_or_default(),
                );
            }
            Event::InputClosed { id } => {
                println!("Input `{id}` was closed");
            }
            other => eprintln!("Received unexpected input: {other:?}"),
        }
    }

    record_results(start, current_size, n, latencies, latency);

    Ok(())
}

fn record_results(
    start: Instant,
    current_size: usize,
    n: u32,
    latencies: Vec<Duration>,
    latency: bool,
) {
    let msg = if latency {
        let avg_latency = latencies.iter().sum::<Duration>() / n;
        format!("size {current_size:<#8x}: {avg_latency:?}")
    } else {
        let duration = start.elapsed();
        let msg_per_sec = n as f64 / duration.as_secs_f64();
        format!("size {current_size:<#8x}: {msg_per_sec:.0} messages per second")
    };
    println!("{msg}");
}