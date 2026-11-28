import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet';
import 'leaflet/dist/leaflet.css';
import '../App.css';

function Map() {
  return (
    <MapContainer
      center={[44.8378, -0.5792]}
      zoom={13}
      style={{ height: '500px', width: '100%' }}
      className='map'
    >
      <TileLayer
        attribution='&copy; OpenStreetMap contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />

      <Marker position={[44.8378, -0.5792]}>
        <Popup>
          Bordeaux
        </Popup>
      </Marker>
    </MapContainer>
  );
}

export default Map;
