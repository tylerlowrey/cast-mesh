import {useEffect, useState} from "react";

const useFetch = (url, options) => {
    const [response, setResponse] = useState(null);
    const [error, setError] = useState(null);
    const [isLoading, setLoading] = useState(false);

    useEffect(  () => {
        const fetchData = async () => {
            setLoading(true);
            console.log("Fetching data...")
            try
            {
                const result = await fetch(url, options);
                const json = await result.json();
                setResponse(json);
                setLoading(false);
            }
            catch(error)
            {
                setError(error);
                console.log(error);
            }
        }
        fetchData();
    }, []);

    return { response, error, isLoading };
}

export default useFetch;