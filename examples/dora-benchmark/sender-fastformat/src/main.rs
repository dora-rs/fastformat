use dora_node_api::{dora_core::config::DataId, DoraNode};
use rand::Rng;

use std::collections::HashMap;
use std::time::Duration;

use fastformat::datatypes::Image;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    for (width, height, c) in &sizes {
        let (width, height, c) = (width.clone(), height.clone(), c.clone());
        let size = (width * height * c) as usize;
        for _ in 0..1000 {
            let data = data.get(&size).unwrap();
            let image = Image::new_bgr8(data.clone(), width, height, None)?;

            node.send_output(
                latency.clone(),
                Default::default(),
                arrow::array::UnionArray::from(image.into_arrow()?),
            )?;
            /*
            node.send_output_raw(latency.clone(), Default::default(), data.len(), |out| {
                out.copy_from_slice(&data);
            })?;
            */

            // sleep a bit to avoid queue buildup
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    // wait a bit to ensure that all throughput messages reached their target
    std::thread::sleep(Duration::from_secs(2));

    // then throughput with full speed
    for (width, height, c) in &sizes {
        let (width, height, c) = (width.clone(), height.clone(), c.clone());
        let size = (width * height * c) as usize;
        for _ in 0..1000 {
            let data = data.get(&size).unwrap();
            let image = Image::new_bgr8(data.clone(), width, height, None)?;

            node.send_output(
                throughput.clone(),
                Default::default(),
                arrow::array::UnionArray::from(image.into_arrow()?),
            )?;

            /*
            node.send_output_raw(throughput.clone(), Default::default(), data.len(), |out| {
                out.copy_from_slice(&data);
            })?;
            */
        }
    }

    Ok(())
}
