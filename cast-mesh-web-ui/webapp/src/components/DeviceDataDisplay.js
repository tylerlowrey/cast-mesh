import React from "react";
import useFetch from "./useFetch";
import {SERVER_URL} from "../constants";
import "../styles/DeviceDataDisplay.css"

const DeviceDataDisplay = ({ device, toggleDeviceDataFunc }) => {

    //const deviceData = useFetch(`${SERVER_URL}/devices/${device?.id || 5}`).response?.data;
    const deviceData = [];

    for (const x of Array(356).keys()) {
        deviceData.push({ timestamp: 1595248537 + (x * 5), count: x})
    }

    console.log(deviceData);

    return(
        <>
            <div className="device-data-display-top">
                <button className="btn btn-secondary" onClick={() => toggleDeviceDataFunc(false)}>
                    &lt; Go Back to Devices
                </button>
                <button className="btn btn-secondary" onClick={() => console.log("Refreshing...")}>
                    <img src="images/refresh_white.png" alt="Refresh button" width="30px"
                        onClick={() => console.log("Refreshing...")}/>
                </button>
            </div>

            <table className="table">
                <thead>
                    <tr>
                        <th scope="col">Timestamp</th>
                        <th scope="col">Count</th>
                    </tr>
                </thead>
                <tbody>
                {deviceData.reverse().map((data, key) => {
                    return (
                        <tr key={key}>
                            <td>{data.timestamp}</td>
                            <td>{data.count}</td>
                        </tr>
                    );
                }) }
                </tbody>
            </table>
        </>
    );
}

export default DeviceDataDisplay;