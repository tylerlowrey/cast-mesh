import React from "react";
import useFetch from "./useFetch";
import {SERVER_URL} from "../constants";

const DeviceDataDisplay = ({ device }) => {

    const deviceData = useFetch(`${SERVER_URL}/devices/${device?.id || 5}`).response?.data;

    console.log(deviceData);

    return(
        <table className="table">
            <tbody>
            {deviceData ? deviceData.map((data, key) => {
                return (
                    <tr key={key}>
                        {data ? Object.entries(data).forEach(([key, value]) => {
                            return (
                                <td key={key}>{value}</td>
                            );
                        }) : null}
                    </tr>
                );
            }) : null}
            </tbody>
        </table>
    );
}

export default DeviceDataDisplay;