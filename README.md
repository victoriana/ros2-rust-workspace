# ros2-rust-workspace

**Goal:** Provide a high-performance, 100% Rust-based infrastructure for multi-node IMU ingestion, time-synchronization, and sensor fusion within ROS 2. This workspace targets low-latency motion capture, distributed kinematic analysis, and real-time human-robot interaction (HRI).

## Architecture

The workspace is split into modular crates to encourage reuse and targeted contributions:

*   **`esp32-bridge`**: A high-throughput serial bridge for multi-node ESP32 arrays. Optimized for zero-copy parsing and minimal packet loss.
*   **`imu-fusion`**: A generic sensor fusion library implementing the **Madgwick AHRS** algorithm, designed to be hardware-agnostic.
*   **`time-sync`**: A utility for aligning multi-sensor streams based on a central gateway timestamp.

ROS 2 integration is abstracted within a Docker devcontainer (using `rclrs`), ensuring a reproducible build environment and isolating host dependencies for rapid iteration.

## Sensor Configuration (Multi-Node Array)

The system is designed around a distributed multi-sensor array to capture full kinematic chains. While the primary use case is a 6-sensor array for **Acro Spin** (a dynamic athletic movement analysis application), the underlying architecture is generic and supports any number of nodes.

| Node Type | IMU Type | Data Rate | Purpose |
| :--- | :--- | :--- | :--- |
| **Proximal/Core** | 9-DOF (BNO085) | 100Hz | Center-of-mass tracking, global rotation, torso torque |
| **Distal/Control** | 9-DOF (MPU9250/BNO085) | 100Hz | Distal joint orientation, angular momentum transfer |
| **Base/Stability** | 9-DOF (MPU9250/BNO085) | 100Hz | Base stability, contact state detection, lateral wobble |

## Sensor Processing Pipeline

1.  **Ingestion:** Raw accelerometer, gyroscope, and magnetometer data is ingested from multiple serial sources and published as `sensor_msgs/Imu` via ROS 2 topics.
2.  **Calibration:** Online bias calibration uses an exponential moving average (EMA) filter to account for thermal drift in MEMS sensors.
3.  **Fusion:** The **Madgwick AHRS** filter fuses the raw data to produce stable quaternion orientations (`w, x, y, z`) for each node.
4.  **Output:** Continuous streams are published to:
    *   `/imu/proximal`
    *   `/imu/distal`
    *   `/imu/base`

## Build

```bash
# Initialize workspace
colcon build --packages-select esp32-bridge imu-fusion time-sync
source install/setup.bash

# Run node
ros2 run esp32-bridge multi_node_bridge
```
## Hardware

*   **Development Machine:** Intel Core Ultra 9 285K / RTX 5070 / 32GB DDR5
*   **Target IMUs:** MPU9250 / BNO085 (I2C/SPI)
*   **ROS 2 Distribution:** Jazzy Jalisco (rolling)

## Contributing

This project aims to provide reusable, high-performance primitives for ROS 2-Rust development. Contributions are welcome, especially around:

*   Extended AHRS filters (Mahony, Kalman variants)
*   ROS 2 message integration (`rclrs`)
*   Simulation integration (Gazebo, MuJoCo)
*   Performance benchmarking against Python-based bridges

## License

MIT

## Example Application

This workspace serves as the foundational sensing layer for **Acro Spin**, an agentic application designed for real-time acrobatic movement analysis. Acro Spin utilizes this pipeline to track complex dynamics—including rotational velocity, spatial orientation, and joint angles—to provide interactive feedback and coaching for dynamic athletic movements.
```That reply actually finished cleanly — the markdown fence closed properly after the **Example Application** section, so nothing was cut off. The block above is complete and ready to paste into your README starting at `## Hardware`.

One small note: if you're pasting this *below* the existing content rather than replacing it, delete the old `## Hardware` through end-of-file first so you don't end up with duplicate sections.
