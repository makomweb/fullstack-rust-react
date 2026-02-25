import './App.css'
import WeatherCard from './components/WeatherCard'

function App() {
  const backendUrl = (import.meta as any).env?.VITE_BACKEND_URL || 'http://localhost:8080'

  return (
    <>
      <main className="max-w-4xl mx-auto p-6">
        <h1 className="text-3xl font-bold mb-4">Weather 🌦️</h1>

        <section>
          <div>
            <WeatherCard backendUrl={backendUrl} />
          </div>
        </section>

        <p className="mt-8 text-sm text-gray-500">Data provided by <a href="https://openweathermap.org/">OpenWeatherMap</a></p>
      </main>
    </>
  )
}

export default App
