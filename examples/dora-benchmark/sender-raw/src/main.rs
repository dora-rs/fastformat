use dora_node_api::{dora_core::config::DataId, DoraNode};
use rand::Rng;

use std::collections::HashMap;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let latency = DataId::from("latency".to_owned());
    let throughput = DataId::from("throughput".to_owned());

    let (mut node, _events) = DoraNode::init_from_env()?;
    let sizes = [
        720 * 480 * 3,
        1280 * 720 * 3,
        1920 * 1080 * 3,
        3840 * 2160 * 3,
    ];

    let mut data = HashMap::new();
    for size in sizes {
        let vec: Vec<u8> = rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(size)
            .collect();

        data.insert(size, vec);
    }

    // test latency first
    for size in sizes {
        for _ in 0..100 {
            let data = data.get(&size).unwrap();

            node.send_output_raw(latency.clone(), Default::default(), data.len(), |out| {
                out.copy_from_slice(&data);
            })?;

            // sleep a bit to avoid queue buildup
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    // wait a bit to ensure that all throughput messages reached their target
    std::thread::sleep(Duration::from_secs(2));

    // then throughput with full speed
    for size in sizes {
        for _ in 0..100 {
            let data = data.get(&size).unwrap();

            node.send_output_raw(throughput.clone(), Default::default(), data.len(), |out| {
                out.copy_from_slice(&data);
            })?;
        }
    }

    Ok(())
}
