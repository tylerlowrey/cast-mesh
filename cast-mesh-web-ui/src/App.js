import React, {useState} from 'react';
import DeviceRegistration from "./components/DeviceRegistration";
import DeviceList from "./components/DeviceList";
import DeviceDataDisplay from "./components/DeviceDataDisplay";
import "./styles/style.css";
import {SERVER_URL} from "./constants";
import useFetch from "./components/useFetch";

function App()
{

    const devices = useFetch(`${SERVER_URL}/devices`).response;
    const [selectedDevice, setSelectedDevice] = useState(null);
    const [showDeviceData, setShowDeviceData] = useState(false);

    console.log(devices);

    const selectDevice = (device) => {
        setSelectedDevice(device);
        setShowDeviceData(true);
    }

    if(showDeviceData)
    {
        return (
            <div className="App">
                <button className="btn btn-secondary" onClick={() => setShowDeviceData(false)}>
                    &lt; Go Back to Devices
                </button>
                <DeviceDataDisplay selectedDevice={selectedDevice}/>
            </div>
        )
    }
    else
    {
        return (
            <div className="App">
                <div className="app-top">
                    <DeviceRegistration/>
                    <DeviceList devices={devices} selectDeviceFunc={selectDevice}/>
                </div>
            </div>
        );
    }


}

export default App;
