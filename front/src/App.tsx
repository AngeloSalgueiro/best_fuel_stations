import Map from './components/Map';
import './App.css'
import useStations from './services/StationsFetcher';

function App() {
  const { stations, dataIsLoaded } = useStations(10);

    if (!dataIsLoaded) return <p>Loading...</p>;


  return (
    <div>
      <Map stations={stations}/>
    </div>
  );
}

export default App;
