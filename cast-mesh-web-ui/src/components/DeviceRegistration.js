import React from 'react';
import "../styles/DeviceRegistration.css";

const DeviceRegistration = () => {
    const handleSubmit = () => {

    }

    return (
        <div className="widget-container">
            <h1 className="device-registration-header">Register Device</h1>
            <form className="device-registration-form" onSubmit={() => handleSubmit()}>
                <label htmlFor="device-name">Device Name</label>
                <input type="text" name="device-name" />
                <label htmlFor="device-address">Device Address</label>
                <input type="text" name="device-address" />
                <label htmlFor="device-port">Device Port</label>
                <input type="number" name="device-port" />
                <div className="device-registration-form-submit-btn">
                    <input type="submit" value="Add Device"/>
                </div>
            </form>
        </div>
    )
}

export default DeviceRegistration;
