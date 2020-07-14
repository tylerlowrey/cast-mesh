# cast-mesh
A tool for collecting data from IoT devices located on a single local network. This project is intended to help with the process of data collection 
between IoT devices that are located on a single local network. It's use of protobuf & gRPC will allow easy customization by generating code stubs
that allow custom implementations based upon the IoT device use-case.

This project was created for my Systems Administration & Security class at Clemson University

## Planned Features
- Provide a TornadoFX based UI that allows for adding an IoT Device that will send data to the collection server
- Provide an easy to implement protobuf-based gRPC protocol for communication between the server and IoT devices
