import React, {useState} from 'react';
import "../styles/DeviceRegistration.css";
import {SERVER_URL} from "../constants";

const DeviceRegistration = () => {

    const [deviceName, setDeviceName] = useState("");
    const [deviceAddress, setDeviceAddress] = useState("");
    const [devicePort, setDevicePort] = useState("");

    const handleSubmit = (event) => {
        let response = fetch(`${SERVER_URL}/devices`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                device_name: deviceName,
                device_address: deviceAddress,
                device_port: Number(devicePort),
            })
        });

        console.log(response.json());

        setDeviceName("");
        setDeviceAddress("");
        setDevicePort("");

        event.preventDefault();
    }

    return (
        <div className="widget-container device-registration-container">
            <h1 className="device-registration-header">Register Device</h1>
            <form className="device-registration-form" onSubmit={handleSubmit}>
                <label htmlFor="device-name">Device Name</label>
                <input type="text" name="device-name" value={deviceName}
                       onChange={ e => setDeviceName(e.target.value)} />
                <label htmlFor="device-address">Device Address</label>
                <input type="text" name="device-address" value={deviceAddress}
                       onChange={ e => setDeviceAddress(e.target.value)} />
                <label htmlFor="device-port">Device Port</label>
                <input type="number" name="device-port" value={devicePort}
                       onChange={e => setDevicePort(e.target.value)}/>
                <div className="device-registration-form-submit-btn">
                    <input type="submit" value="Add Device"/>
                </div>
            </form>
        </div>
    )
}

export default DeviceRegistration;
