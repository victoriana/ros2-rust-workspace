# ros2-rust-workspace

**Goal:** Provide a high-performance, 100% Rust-based infrastructure for IMU ingestion, time-synchronization, and sensor fusion within ROS 2. This workspace targets low-latency motion capture, distributed kinematic analysis, and real-time human-robot interaction (HRI).

## Architecture

The workspace is split into modular crates to encourage reuse and targeted contributions:

*   **`esp32-bridge`**: A high-throughput serial bridge for multi-node ESP32 arrays. Optimized for zero-copy parsing and minimal packet loss. Accepts N=1 serial port gracefully; multi-node mode activates when ≥2 ports are configured.
*   **`imu-fusion`**: A generic sensor fusion library implementing the **Madgwick AHRS** algorithm, designed to be hardware-agnostic. Intended for use with raw-output IMUs (accel/gyro/mag); not required when the sensor performs onboard fusion (see Sensor Processing Pipeline).
*   **`time-sync`**: A utility for aligning multi-sensor streams based on a central gateway timestamp. Inert (no-op) in single-node configurations.

ROS 2 integration is abstracted within a Docker devcontainer (using `rclrs`), ensuring a reproducible build environment and isolating host dependencies for rapid iteration.

## Sensor Configuration

The system supports any number of nodes (N ≥ 1). The current deployment is a **single-node waist unit**; the three-node taxonomy below is the expansion target for full kinematic-chain capture.

| Node Type | IMU Type | Data Rate | Purpose | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Waist / Core** | 9-DOF (BNO055) | 100Hz | Center-of-mass tracking, global rotation (yaw), axial lean | ✅ Active |
| **Proximal/Distal** | 9-DOF (BNO055) | 100Hz | Distal joint orientation, angular momentum transfer | Planned |
| **Base/Stability** | 9-DOF (BNO055) | 100Hz | Base stability, contact state detection, lateral wobble | Planned |

## Sensor Processing Pipeline

**Note on fusion strategy:** The BNO055 performs Bosch's sensor fusion internally on an onboard Cortex-M0+ and exposes fused quaternion/Euler output at 100Hz. Running Madgwick on already-fused data is redundant and incorrect if fed Euler angles. Two supported paths:

*   **Path A — Onboard fusion (current, BNO055):** Read the sensor's native quaternion directly and publish as `sensor_msgs/Imu` with covariance placeholders. `imu-fusion` is bypassed.
*   **Path B — Host-side fusion (raw-output IMUs):** Ingest raw accel/gyro/mag and run **Madgwick AHRS** in `imu-fusion` to produce quaternions. Required for sensors without onboard fusion (e.g., LSM6DS3 + external magnetometer).

Pipeline stages:

1.  **Ingestion:** Serial frames from each ESP32 node are parsed and published as `sensor_msgs/Imu` via ROS 2 topics (`/imu/waist`, `/imu/proximal`, `/imu/distal`, `/imu/base`).
2.  **Calibration:** Online bias calibration uses an exponential moving average (EMA) filter to account for thermal drift in MEMS sensors.
3.  **Fusion:** Per Path A or Path B above.
4.  **Output:** Continuous quaternion streams published per active topic.

## Build

```bash
# Initialize workspace
colcon build --packages-select esp32-bridge imu-fusion time-sync
source install/setup.bash

# Run node (single or multi-port config)
ros2 run esp32-bridge multi_node_bridge
```
## Hardware

*   **Development Machine:** Intel Core Ultra 9 285K / RTX 5070 / 32GB DDR5
*   **Target IMUs:** BNO055 (I2C) — onboard fusion (Path A)
*   **ROS 2 Distribution:** Jazzy Jalisco (rolling)## Contributing

This project aims to provide reusable, high-performance primitives for ROS 2-Rust development. Contributions are welcome, especially around:

*   Extended AHRS filters (Mahony, Kalman variants) for Path B deployments
*   ROS 2 message integration (`rclrs`)
*   Simulation integration (Gazebo, MuJoCo)
*   Performance benchmarking against Python-based bridges

## License

MIT

## Example Application

This workspace serves as the foundational sensing layer for **Acro Spin**, an agentic application designed for real-time acrobatic movement analysis. Acro Spin utilizes this pipeline to track complex dynamics—including rotational velocity, spatial orientation, and axial lean—to provide interactive feedback and coaching for dynamic athletic movements. The initial validation target is pirouette analysis via a single waist-mounted node.
