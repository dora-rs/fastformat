use dora_node_api::{dora_core::config::DataId, DoraNode, Event};
use rand::Rng;

use std::collections::HashMap;

use clap::Parser;

use eyre::Result;

use std::time::Duration;
use std::time::Instant;

use fastformat::Image;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    receiver: bool,

    #[clap(long)]
    sender: bool,

    #[clap(long)]
    raw: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (r, s, raw) = (args.receiver, args.sender, args.raw);

    match (r, s) {
        (true, false) => receiver(raw),
        (false, true) => sender(raw),
        _ => Err(eyre::eyre!(
            "Exactly one of --receiver or --sender must be set"
        )),
    }
}

fn sender(raw: bool) -> Result<()> {
    let latency = DataId::from("latency".to_owned());
    let throughput = DataId::from("throughput".to_owned());

    let (mut node, _events) = DoraNode::init_from_env()?;

    let sizes: [(u32, u32, u32); 4] = [
        (720, 480, 3),
        (1280, 720, 3),
        (1920, 1080, 3),
        (3840, 2160, 3),
    ];

    let mut data = HashMap::new();
    for (width, height, c) in &sizes {
        let size = (width * height * c) as usize;
        let vec: Vec<u8> = rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(size)
            .collect();

        data.insert(size, vec);
    }

    // test latency first
    for &(width, height, c) in &sizes {
        let size = (width * height * c) as usize;
        for _ in 0..300 {
            if raw {
                let data = data.get(&size).unwrap();

                node.send_output_raw(latency.clone(), Default::default(), data.len(), |out| {
                    out.copy_from_slice(data);
                })?;
            } else {
                let data = data.get(&size).unwrap();
                let image = Image::new_bgr8(data.clone(), width, height, None)?;
                let arrow = image.into_arrow()?;

                node.send_output(
                    latency.clone(),
                    Default::default(),
                    arrow::array::UnionArray::from(arrow),
                )?;
            }

            // sleep a bit to avoid queue buildup
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    // wait a bit to ensure that all throughput messages reached their target
    std::thread::sleep(Duration::from_secs(2));

    // then throughput with full speed
    for &(width, height, c) in &sizes {
        let size = (width * height * c) as usize;
        for _ in 0..300 {
            if raw {
                let data = data.get(&size).unwrap();

                node.send_output_raw(throughput.clone(), Default::default(), data.len(), |out| {
                    out.copy_from_slice(data);
                })?;
            } else {
                let data = data.get(&size).unwrap();
                let image = Image::new_bgr8(data.clone(), width, height, None)?;
                let arrow = image.into_arrow()?;

                node.send_output(
                    throughput.clone(),
                    Default::default(),
                    arrow::array::UnionArray::from(arrow),
                )?;
            }
        }
    }

    Ok(())
}

fn receiver(raw: bool) -> Result<()> {
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
                let data_len = match raw {
                    true => data.len(),
                    false => {
                        use arrow::array::Array;

                        let image_raw = Image::raw_data(data.0.into_data())?;
                        let image = Image::view_from_raw_data(&image_raw)?;

                        image.data.len()
                    }
                };

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
