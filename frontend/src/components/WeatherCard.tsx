import { useEffect, useState } from 'react'

type Props = {
  backendUrl?: string
}

type WeatherResp = {
  location: string
  temp_c: number
  description?: string | null
  humidity?: number | null
  pressure?: number | null
  wind_speed?: number | null
  wind_deg?: number | null
  icon?: string | null
  sunrise?: number | null
  sunset?: number | null
  coord?: { lat: number; lon: number } | null
}

export default function WeatherCard({ backendUrl }: Props) {
  const backend = backendUrl || (import.meta as any).env?.VITE_BACKEND_URL || 'http://localhost:8080'
  const [data, setData] = useState<WeatherResp | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    setLoading(true)
    fetch(`${backend}/weather`)
      .then(async (r) => {
        if (!r.ok) throw new Error(`${r.status} ${await r.text()}`)
        return r.json()
      })
      .then((d: WeatherResp) => {
        setData(d)
        setError(null)
      })
      .catch((e) => {
        console.error('WeatherCard error', e)
        setError(String(e))
      })
      .finally(() => setLoading(false))
  }, [backend])

  if (loading) {
    return (
      <div className="p-4 bg-white shadow rounded-lg inline-block text-center max-w-xs">
        <div className="text-sm text-gray-500">Berlin</div>
        <div className="text-xl font-semibold">-- °C</div>
        <div className="text-xs text-gray-400">Loading…</div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="p-4 bg-red-50 text-red-700 shadow rounded-lg inline-block">
        <div className="text-sm">Error</div>
        <div className="text-xs">{error}</div>
      </div>
    )
  }

  if (!data) return null

  const sunrise = data.sunrise ? new Date(data.sunrise * 1000).toLocaleTimeString() : '--'
  const sunset = data.sunset ? new Date(data.sunset * 1000).toLocaleTimeString() : '--'

  return (
    <div className="p-4 bg-white shadow rounded-lg inline-block text-center max-w-sm">
      <div className="flex items-center gap-3 justify-center">
        {data.icon ? (
          <img src={`https://openweathermap.org/img/wn/${data.icon}@2x.png`} alt={data.description || 'weather'} className="inline-block w-12 h-12" />
        ) : null}
        <div>
          <div className="text-sm text-gray-500">{data.location}</div>
          <div className="text-3xl font-bold">{Math.round(data.temp_c)}°C</div>
          {data.description ? <div className="text-sm text-gray-600">{data.description}</div> : null}
        </div>
      </div>

      <div className="mt-3">
        <table className="w-full text-left text-sm">
          <tbody>
            <tr>
              <td className="pr-2 font-medium">Humidity</td>
              <td>{data.humidity != null ? `${data.humidity}%` : '--'}</td>
            </tr>
            <tr>
              <td className="pr-2 font-medium">Pressure</td>
              <td>{data.pressure != null ? `${data.pressure} hPa` : '--'}</td>
            </tr>
            <tr>
              <td className="pr-2 font-medium">Wind</td>
              <td>{data.wind_speed != null ? `${data.wind_speed} m/s ${data.wind_deg ? data.wind_deg + '°' : ''}` : '--'}</td>
            </tr>
            <tr>
              <td className="pr-2 font-medium">Sunrise / Sunset</td>
              <td>{sunrise} / {sunset}</td>
            </tr>
            <tr>
              <td className="pr-2 font-medium">Coords</td>
              <td>{data.coord ? `${data.coord.lat.toFixed(3)}, ${data.coord.lon.toFixed(3)}` : '--'}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  )
}
