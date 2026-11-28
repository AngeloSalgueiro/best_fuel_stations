import { MapContainer, TileLayer, Marker, Popup } from "react-leaflet";
import "leaflet/dist/leaflet.css";
import "../App.css";
import type Station from "../entities/Station";

function Map({ stations }: { stations: Station[] }) {
  let markers = stations.map((station) => (
    <Marker
      key={station.id}
      position={[station.latitude, station.longitude]}
    >
      <Popup>
        <div>
          <strong>{station.city}</strong>
          <br/>
          {station.address}
          <br/>
          {station.cp}
        </div>
      </Popup>
    </Marker>
  ));

  return (
    <MapContainer
      center={[46.221, 5.245]}
      zoom={13}
      style={{ height: "500px", width: "100%" }}
      className="map"
    >
      <TileLayer
        attribution="&copy; OpenStreetMap contributors"
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />

      {markers}
    </MapContainer>
  );
}

export default Map;
