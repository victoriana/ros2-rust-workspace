# ros2-rust-workspace

**Goal:** Build a 100% Rust-based sensor processing pipeline for an acrobot system designed for human movement training (cartwheels, pirouettes, etc.).

## Architecture

The workspace consists of a single Rust crate (sensor_processing) that handles IMU sensor fusion and orientation estimation. ROS 2 integration via rclrs is abstracted inside a Docker devcontainer, keeping the host environment clean for rapid iteration.

## Sensor Configuration (Target)

| Body Part | IMU | Purpose |
|---|---|---|
| Wrists (2x) | 9-DOF (MPU9250/BNO085) | Hand placement timing, rotation rate |
| Ankles (2x) | 9-DOF | Dorsiflexion/plantarflexion, foot strike detection |
| Waist (1x) | 9-DOF | Whole-body yaw rate, pirouette spin counting |

## Sensor Processing Pipeline

1. Raw IMU data is published as sensor_msgs/Imu via ROS 2
2. Bias calibration uses exponential moving average filter
3. Madgwick AHRS filter produces quaternion orientation (roll, pitch, yaw)
4. Output topics: /imu/wrist_left, /imu/wrist_right, /imu/ankle_left, /imu/ankle_right, /imu/waist

## Build

```bash
# Inside devcontainer
colcon build --packages-select sensor_processing
source install/setup.bash
```
## Hardware

- Development machine: Intel Core Ultra 9 285K / RTX 5070 / 32GB DDR5
- Target IMUs: MPU9250 / BNO085 (I2C/SPI)
- ROS 2 Distribution: Jazzy Jalisco (rolling)

## Contributing

This project is a learning exercise in Rust-based robotic systems programming. Contributions welcome, especially around:

- Extended AHRS filters (Mahony, Kalman variants)
- ROS 2 message integration (rclrs)
- Simulation integration (Gazebo, MuJoCo)

## License

MIT
