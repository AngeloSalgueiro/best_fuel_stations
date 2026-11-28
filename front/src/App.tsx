import Map from "./components/Map";
import "./App.css";
import useStations from "./services/StationsFetcher";
import { useEffect, useState } from "react";

function App() {
  const [coordinates, setCoordinates] = useState(null);

  useEffect(() => {
    if (!navigator.geolocation) {
      console.error("Geolocation is not supported");
      return;
    }

    const watchId = navigator.geolocation.watchPosition(
      (position) => {
        setCoordinates({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
        });
      },
      (error) => {
        console.error("Geolocation error:", error);
      }
    );

    return () => {
      navigator.geolocation.clearWatch(watchId);
    };
  }, []);

  return (
    <div>
      {coordinates ? (
        <StationsMap
          latitude={coordinates.latitude}
          longitude={coordinates.longitude}
        />
      ) : (
        <p>Getting your location...</p>
      )}
    </div>
  );
}

function StationsMap({ latitude, longitude }) {
  const { stations, dataIsLoaded } = useStations(
    latitude,
    longitude,
    10
  );

  if (!dataIsLoaded) {
    return <p>Loading stations...</p>;
  }

  return <Map stations={stations} latitude={latitude} longitude={longitude} />;
}

export default App;
