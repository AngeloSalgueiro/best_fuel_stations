import { useEffect, useState } from "react";
import type Station from "../entities/Station";

function useStations(latitude: number, longitude: number, nb_stations: number) {
  const [stations, setStations] = useState<Station[]>([]);
  const [dataIsLoaded, setDataIsLoaded] = useState(false);

  useEffect(() => {
    const fetchStations = async () => {
      try {
        const response = await fetch(
          "http://127.0.0.1:3000/api/v1/stations/nearest/" + nb_stations,
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              latitude: latitude,
              longitude: longitude,
            }),
          },
        );

        if (!response.ok) {
          throw new Error("Failed to fetch stations");
        }

        const json: Station[] = await response.json();
        setStations(json);
      } catch (error) {
        console.error(error);
      } finally {
        setDataIsLoaded(true);
      }
    };

    fetchStations();
  }, [nb_stations]);

  return { stations, dataIsLoaded };
}

export default useStations;
