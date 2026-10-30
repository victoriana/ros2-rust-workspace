# ros2-rust-workspace

**Goal:** Build a 100% Rust-based multi-IMU sensor processing pipeline for real-time orientation estimation, targeting low-latency motion capture and human-robot interaction applications.

## Architecture

The workspace consists of a single Rust crate (sensor_processing) that handles IMU sensor fusion and orientation estimation. ROS 2 integration via rclrs is abstracted inside a Docker devcontainer, keeping the host environment clean for rapid iteration.

## Sensor Configuration (Target)

| Sensor Position | IMU | Purpose |
|---|---|---|
| Wrist (2×) | 9-DOF (MPU9250/BNO085) | Distal joint orientation, rotation rate |
| Ankle (2×) | 9-DOF | Joint angle estimation, contact detection |
| Waist (1×) | 9-DOF | Trunk orientation, whole-body rotation rate |

## Sensor Processing Pipeline

1. Raw IMU data is published as sensor_msgs/Imu via ROS 2
2. Bias calibration uses exponential moving average filter
3. Madgwick AHRS filter produces quaternion orientation (roll, pitch, yaw)
4. Output topics: /imu/wrist_left, /imu/wrist_right, /imu/ankle_left, /imu/ankle_right, /imu/waist

## Build

```bash
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

## Example Application

This workspace is being developed alongside a wearable IMU rig for gymnastics movement analysis (handstand entry timing, rotational velocity tracking). See `acrobot-sensors` for the application-specific implementation.
