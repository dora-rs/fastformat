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

- **Image** as a **UnionArray**,
    - Field 0: Uint32Array [width, height] (e.g [1280, 720])
    - Field 1: StringArray [encoding] (e.g ["RGB8"])
    - Field 2: UintXArray [data] (e.g [0, 255, 0, 255, 0, 255, ...])
    - Field 3 (Optional): StringArray [name] (e.g ["image.front_camera"])

- **ImageSequence** as a **UnionArray**,
    - Field 0: StringArray [path_to_frames] (e.g ["path/to/frames/"])
    - Field 1: Uint32Array [framerate] (e.g [30])
    - Field 2 (Optional): StringArray [name] (e.g ["video.front_camera"])

- **BBoxes** as a **UnionArray**,
    - Field 0: Int32Array [data] (e.g [x1, y1, x2, y2, x1, y1, x2, y2, ...])
    - Field 1: Float32Array [confidence] (e.g [0.9, 0.8, ...])
    - Field 2: StringArray [label] (e.g ["car", "person", ...])
    - Field 3: StringArray [encoding] (e.g ["XYXY"] or ["XYWH"])

- **LabelledValues** as a **StructArray*,
    - Field "labels": StringArray [labels] (e.g ["head", "neck", ...])
    - Field "values": XYZWArray [values] (e.g [a, b, c, d, ...])