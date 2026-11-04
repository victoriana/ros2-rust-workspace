# ros2-rust-workspace

**Goal:** Develop a 100% Rust-based, multi-node IMU sensor fusion pipeline for real-time 6-DOF orientation estimation. This workspace targets low-latency motion capture, distributed kinematic analysis, and human-robot interaction (HRI) applications.

## Architecture

The workspace is a single Rust crate (`sensor_processing`) that handles high-frequency IMU data ingestion, bias calibration, and attitude estimation via the **Madgwick AHRS** algorithm. ROS 2 integration is abstracted within a Docker devcontainer (using `rclrs`), ensuring a reproducible build environment and isolating host dependencies for rapid iteration.

## Sensor Configuration (6-Node Array)

The system is designed around a distributed 6-sensor array to capture the full kinematic chain of rotational maneuvers.

| Sensor Node | IMU Type | Data Rate | Purpose |
| :--- | :--- | :--- | :--- |
| **Head (1×)** | 9-DOF (BNO085) | 100Hz | Axis stabilization, spotting coordination, head-centric reference frame |
| **Waist (1×)** | 9-DOF (BNO085) | 100Hz | Center-of-mass (CoM) tracking, torso torque, global rotation rate |
| **Wrist (2×)** | 9-DOF (MPU9250/BNO085) | 100Hz | Distal joint orientation, angular momentum transfer (the "tuck") |
| **Ankle (2×)** | 9-DOF (MPU9250/BNO085) | 100Hz | Base stability, contact state detection, lateral wobble analysis |

## Sensor Processing Pipeline

1. **Ingestion:** Raw accelerometer, gyroscope, and magnetometer data is published as `sensor_msgs/Imu` via ROS 2 topics.
2. **Calibration:** Online bias calibration uses an exponential moving average (EMA) filter to account for thermal drift in MEMS sensors.
3. **Fusion:** The **Madgwick AHRS** filter fuses the raw data to produce stable quaternion orientations (`w, x, y, z`) for each node.
4. **Output:** Continuous streams are published to:
   * `/imu/head`, `/imu/waist` (Proximal/Core nodes)
   * `/imu/wrist_left`, `/imu/wrist_right` (Distal/Control nodes)
   * `/imu/ankle_left`, `/imu/ankle_right` (Base/Stability nodes)

## Build

```bash
# Initialize workspace
colcon build --packages-select sensor_processing
source install/setup.bash

# Run node
ros2 run sensor_processing imu_fusion_node
```
## Hardware

- **Development Machine:** Intel Core Ultra 9 285K / RTX 5070 / 32GB DDR5
- **Target IMUs:** MPU9250 / BNO085 (I2C/SPI)
- **ROS 2 Distribution:** Jazzy Jalisco (rolling)

## Contributing

This project is a learning exercise in Rust-based robotic systems programming. Contributions are welcome, especially around:

- Extended AHRS filters (Mahony, Kalman variants)
- ROS 2 message integration (`rclrs`)
- Simulation integration (Gazebo, MuJoCo)

## License

MIT

## Example Application

This workspace serves as the foundational sensing layer for **Acro Spin**, an agentic application designed for real-time acrobatic movement analysis. Acro Spin utilizes this pipeline to track complex dynamics—including rotational velocity, spatial orientation, and joint angles—to provide interactive feedback and coaching for dynamic athletic movements.
