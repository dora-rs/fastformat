## Project Objective

The goal is to create an efficient way to reconstruct and process data in real-time, in specific formats like NDarray,
Numpy, or Arrow, without unnecessary copies.

- **Independent Library**: We are creating an independent library for our formats, which everyone could use even without
  DORA. The current name is fastformat: <https://github.com/dora-rs/fastformat>

- **Agnosticism**: We want our solution to be agnostic to Arrow, meaning it can work with Numpy and other data types
  easily by implementing to_[format] functions.

- **Rust and Python Code**: The main implementation will be in Rust for performance and portability reasons, with a
  Python interface using PyO3. This will allow for fast vector manipulation and potentially GPU support in the future.

- **Portability and Dependencies**: By using Rust, we aim to minimize dependencies and maximize portability. This will
  allow us to achieve optimal performance across different platforms.

- **Simplicity**: The goal is not to create yet another library for creating formats to store/process data. The aim here
  is to create a simple interface to wrap data from one type and convert it to another.

## DataTypes

- **Image**: (Arrow representation is a **UnionArray**),
    - Field "data": UintXArray (e.g [0, 255, 0, 255, 0, 255, ...])
    - Field "width": Uint32Array (e.g [1280])
    - Field "height": Uint32Array (e.g [720])
    - Field "encoding": StringArray (e.g ["RGB8"])
    - Field "name" (Optional): StringArray (e.g,["image.front_camera"] or [None])

- **BBox**: (Arrow representation is a **UnionArray**),
    - Field "data": Float32Array (e.g [0.0f32, 1.0f32, ...])
    - Field "confidence": Float32Array (e.g [0.98f32, 0.76f32, ...])
    - Field "label": StringArray (e.g ["cat", "car", ..."])
    - Field "encoding": StringArray (e.g ["XYXY"] or ["XYWH"])
