import React from "react";
import "../styles/DeviceList.css";

const DeviceList = ({ devices, selectDeviceFunc }) => {
    return(
        <div className="device-list-container">
            <h1>Devices</h1>
            <div className="device-list-widget-container">

                { devices ? devices.map((device, key) => {
                    return (
                        <div className="widget-container device-list-widget" key={key}>
                            <div className="input-group">
                                <div className="input-group-prepend">
                                    <span className="input-group-text" id="basic-addon1">Device Name</span>
                                </div>
                                <input type="text" className="form-control" placeholder={device.device_name} aria-label="Device-Name"
                                       aria-describedby="basic-addon1" disabled />
                            </div>
                            <div className="input-group">
                                <div className="input-group-prepend">
                                    <span className="input-group-text" id="basic-addon1">Device Address</span>
                                </div>
                                <input type="text" className="form-control" placeholder={device.device_address} aria-label="Device-Address"
                                       aria-describedby="basic-addon1" disabled />
                            </div>
                            <div className="input-group">
                                <div className="input-group-prepend">
                                    <span className="input-group-text" id="basic-addon1">Device Port</span>
                                </div>
                                <input type="text" className="form-control" placeholder={device.device_port} aria-label="Device-Port"
                                       aria-describedby="basic-addon1" disabled />
                            </div>
                            <button type="button" className="btn btn-danger device-list-button">
                                Remove Device
                            </button>
                            <button type="button" className="btn btn-primary device-list-button" onClick={ () => selectDeviceFunc(device)}>
                                View Data
                            </button>
                        </div>
                    );
                }) : null}
            </div>
        </div>

    )
}

export default DeviceList;