# 🚀 **fastformat: High-Performance Data Processing Library**

[![Build Status](https://img.shields.io/github/workflow/status/dora-rs/fastformat/CI)](https://github.com/dora-rs/fastformat/actions)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/github/v/tag/dora-rs/fastformat)](https://github.com/dora-rs/fastformat/tags)

## 🎯 **Project Objective**

The goal of **fastformat** is to build an **efficient**, **real-time** data processing library that supports formats like **NDarray**, **Numpy**, and **Arrow**, without unnecessary data copies. ⚡

This independent library enables **simple and fast** data conversion between formats, ensuring optimal performance across various platforms.

🌟 Key features of **fastformat**:
- **💼 Independent Library**: Usable with or without [**DORA**](https://github.com/dora-rs). Find the repo [here](https://github.com/dora-rs/fastformat).
- **🌐 Agnostic Format**: The library is designed to support various data formats like **Numpy**, **Arrow**, and others, with conversion through `into_[format]` functions.
- **🦀 Rust & 🐍 Python Integration**: The core is implemented in **Rust** for speed and portability, with a Python interface using **PyO3** for ease of use and compatibility.
- **📦 Minimal Dependencies**: Built with **Rust**, fastformat ensures minimal external dependencies and maximum cross-platform compatibility.
- **🔄 Simplicity in Conversion**: fastformat doesn’t aim to handle complex data operations on its own. Instead, it provides a simple interface to wrap and convert data types efficiently, leaving complex operations to other specialized projects.

> **Note**: fastformat is **not** designed to be a fully-featured API for performing advanced operations on specific data types. Instead, it focuses on providing **simple interfaces** for handling data representations in various formats.

---

## 💻 **Technology Stack**

- **Rust** 🦀 for core functionality and high-performance processing.
- **PyO3** 🐍 for seamless integration with Python.
- **Arrow** 🏹 for powerful in-memory data representation.
- **Kornia-rs** 🖼️ as an **OpenCV replacement** in Rust for advanced image processing when needed.

---

## 🚧 **Installation Instructions**

### Rust

```Cargo.toml
[dependencies]
fastformat = { version = "0.1.0" }
```

### Python

Every `pip-compatible` package manager can be used to install **fastformat**. Here’s an example using `pip`:

```bash
pip install fastformat
```

**Note**: We encourage using `uv pip` inside a virtual environment to avoid conflicts with system packages.

---

## 📚 **Usage Example**

Here’s a simple example of how to use **fastformat** to convert data formats:

```python
import fastformat

# Create a Rust/Python native Image
my_image = fastformat.datatypes.image.new_rgb8([0, 0, 0], 1, 1, "My Image")

# Example: Convert NDarray to Arrow
arrow_data = my_image.into_arrow()

# Example: Convert Arrow to Numpy
numpy_data = my_image.into_numpy()
```

---

## 📦 **Future Plans**

- GPU Support for faster processing with **CUDA** ⚡
- Extend support to more formats (e.g. **Torch Tensors**, **Pandas DataFrames**).
- Add support for **multithreading** and **distributed computing**.

---

## 🙌 **Contributing**

We welcome contributions! Feel free to submit issues or pull requests. Check the [CONTRIBUTING](CONTRIBUTING.md) guide for more information.

---

## 📜 **License**

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

🚀 Happy coding with **fastformat**!
