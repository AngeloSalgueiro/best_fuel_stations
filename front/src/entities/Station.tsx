import type Fuel from "./Fuel";

export default interface Station{
  id: number,
  latitude: number,
  longitude: number,
  city: string,
  address: string,
  cp: string,
  fuels: Fuel[],
}
