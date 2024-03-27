[![LicenseBadge]][License]
[![DocsBadge]][Docs]
[![build](https://github.com/charles-r-earp/autograph/actions/workflows/ci.yml/badge.svg)](https://github.com/charles-r-earp/autograph/actions/workflows/ci.yml)

[License]: https://github.com/charles-r-earp/autograph/blob/main/LICENSE-APACHE
[LicenseBadge]: https://img.shields.io/badge/license-MIT/Apache_2.0-blue.svg
[Docs]: https://docs.rs/autograph
[DocsBadge]: https://docs.rs/autograph/badge.svg

# autograph

A machine learning library for Rust.

GPGPU kernels implemented with [krnl](https://github.com/charles-r-earp/krnl).

- Host and device execution.
- Tensors emulate [ndarray](https://github.com/rust-ndarray/ndarray)
  - Host tensors can be borrowed as arrays.
- Tensors, models, and optimizers can be serialized with [serde](https://github.com/serde-rs/serde).
  - Portable between platforms.
  - Save and resume training progress.
- Fully extensible, in Rust.

## Neural Networks

```rust
#[derive(Layer, Forward)]
#[autograph(forward(Variable4, Output=Variable2))]
struct LeNet5 {
    conv1: Conv2,
    relu1: Relu,
    pool1: MaxPool2,
    conv2: Conv2,
    relu2: Relu,
    pool2: MaxPool2,
    flatten: Flatten,
    dense1: Dense,
    relu3: Relu,
    dense2: Dense,
    relu4: Relu,
    dense3: Dense,
}

impl LeNet5 {
    fn new(device: Device, scalar_type: ScalarType) -> Result<Self> {
        let conv1 = Conv2::builder()
            .device(device.clone())
            .scalar_type(scalar_type)
            .inputs(1)
            .outputs(6)
            .filter([5, 5])
            .build()?;
        let relu1 = Relu;
        let pool1 = MaxPool2::builder().filter([2, 2]).build();
        let conv2 = Conv2::builder()
            .device(device.clone())
            .scalar_type(scalar_type)
            .inputs(6)
            .outputs(16)
            .filter([5, 5])
            .build()?;
        let relu2 = Relu;
        let pool2 = MaxPool2::builder().filter([2, 2]).build();
        let flatten = Flatten;
        let dense1 = Dense::builder()
            .device(device.clone())
            .scalar_type(scalar_type)
            .inputs(16 * 4 * 4)
            .outputs(128)
            .build()?;
        let relu3 = Relu;
        let dense2 = Dense::builder()
            .device(device.clone())
            .scalar_type(scalar_type)
            .inputs(128)
            .outputs(84)
            .build()?;
        let relu4 = Relu;
        let dense3 = Dense::builder()
            .device(device.clone())
            .scalar_type(scalar_type)
            .inputs(84)
            .outputs(10)
            .bias(true)
            .build()?;
        Ok(Self {
            conv1,
            relu1,
            pool1,
            conv2,
            relu2,
            pool2,
            flatten,
            dense1,
            relu3,
            dense2,
            relu4,
            dense3,
        })
    }
}

let mut model = LeNet5::new(device.clone(), ScalarType::F32)?;
model.set_training(true)?;
let y = model.forward(x)?;
let loss = y.cross_entropy_loss(t)?;
loss.backward()?;
model.try_for_each_parameter_view_mut(|parameter| {
    optimizer.update(learning_rate, parameter)
})?;
```

See the [Neural Network MNIST](examples/neural-network-mnist) example.

# Benchmarks

_NVIDIA GeForce GTX 1060 with Max-Q Design_

## LeNet5(training, batch_size = 100)

|                   | `autograph`                | `tch`                            |
| :---------------- | :------------------------- | :------------------------------- |
| **`bf16_host`**   | `482.80 ms` (✅ **1.00x**) | `75.30 ms` (🚀 **6.41x faster**) |
| **`f32_host`**    | `5.44 ms` (✅ **1.00x**)   | `3.09 ms` (✅ **1.76x faster**)  |
| **`bf16_device`** | `1.76 ms` (✅ **1.00x**)   | `17.99 ms` (❌ _10.20x slower_)  |
| **`f32_device`**  | `1.75 ms` (✅ **1.00x**)   | `1.20 ms` (✅ **1.45x faster**)  |

## LeNet5(inference, batch_size = 1,000)

|                   | `autograph`               | `tch`                             |
| :---------------- | :------------------------ | :-------------------------------- |
| **`bf16_host`**   | `1.78 s` (✅ **1.00x**)   | `192.75 ms` (🚀 **9.25x faster**) |
| **`f32_host`**    | `12.23 ms` (✅ **1.00x**) | `9.57 ms` (✅ **1.28x faster**)   |
| **`bf16_device`** | `4.62 ms` (✅ **1.00x**)  | `48.72 ms` (❌ _10.54x slower_)   |
| **`f32_device`**  | `4.76 ms` (✅ **1.00x**)  | `1.84 ms` (🚀 **2.58x faster**)   |

See the [Neural Network](benches/neural-network-benches) benchmark.

# License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed except according to those terms.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
