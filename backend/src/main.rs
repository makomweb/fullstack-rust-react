use axum::{routing::get, Json, Router, http::StatusCode};
use serde::{Serialize, Deserialize};
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use std::env;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

// GET /health  →  { "status": "ok" }
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[derive(Serialize)]
struct WeatherResponse {
    location: String,
    temp_c: f64,
    description: Option<String>,
    humidity: Option<u64>,
    pressure: Option<u64>,
    wind_speed: Option<f64>,
    wind_deg: Option<u64>,
    icon: Option<String>,
    sunrise: Option<u64>,
    sunset: Option<u64>,
    coord: Option<Coord>,
}

#[derive(Serialize)]
struct Coord {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize)]
struct OwmMain {
    temp: f64,
    humidity: Option<u64>,
    pressure: Option<u64>,
}

#[derive(Deserialize)]
struct OwmWeather {
    description: Option<String>,
    icon: Option<String>,
}

#[derive(Deserialize)]
struct OwmWind {
    speed: Option<f64>,
    deg: Option<u64>,
}

#[derive(Deserialize)]
struct OwmSys {
    sunrise: Option<u64>,
    sunset: Option<u64>,
}

#[derive(Deserialize)]
struct OwmCoord {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize)]
struct OwmResp {
    main: OwmMain,
    weather: Option<Vec<OwmWeather>>,
    wind: Option<OwmWind>,
    sys: Option<OwmSys>,
    coord: Option<OwmCoord>,
    name: Option<String>,
}


// GET /weather → { location: "Berlin", temp_c: 12.3 }
async fn weather() -> Result<Json<WeatherResponse>, (StatusCode, String)> {
    info!("GET /weather called");
    let api_key = env::var("OPENWEATHER_API_KEY")
        .map_err(|_| {
            error!("OPENWEATHER_API_KEY not set");
            (StatusCode::INTERNAL_SERVER_ERROR, "OPENWEATHER_API_KEY not set".to_string())
        })?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q=Berlin,DE&units=metric&appid={}",
        api_key
    );

    // Avoid logging the raw URL with the API key; log only that a request is being made.
    info!("Requesting OpenWeather for Berlin (units=metric); api_key present: {}", !api_key.is_empty());
    let resp = reqwest::get(&url).await.map_err(|e| {
        error!("reqwest error: {}", e);
        (StatusCode::BAD_GATEWAY, format!("reqwest error: {}", e))
    })?;

    let status = resp.status();
    if !status.is_success() {
        let txt = resp.text().await.unwrap_or_default();
        error!("OpenWeather returned error status: {} - body: {}", status, txt);
        return Err((StatusCode::BAD_GATEWAY, format!("OpenWeather error: {}", txt)));
    }

    let o: OwmResp = resp.json().await.map_err(|e| {
        error!("json parse error: {}", e);
        (StatusCode::BAD_GATEWAY, format!("json parse: {}", e))
    })?;

    info!("OpenWeather returned temp: {} C", o.main.temp);

    Ok(Json(WeatherResponse {
        location: o.name.clone().unwrap_or_else(|| "Berlin".to_string()),
        temp_c: o.main.temp,
        description: o.weather.as_ref().and_then(|v| v.get(0)).and_then(|w| w.description.clone()),
        humidity: o.main.humidity,
        pressure: o.main.pressure,
        wind_speed: o.wind.as_ref().and_then(|w| w.speed),
        wind_deg: o.wind.as_ref().and_then(|w| w.deg),
        icon: o.weather.as_ref().and_then(|v| v.get(0)).and_then(|w| w.icon.clone()),
        sunrise: o.sys.as_ref().and_then(|s| s.sunrise),
        sunset: o.sys.as_ref().and_then(|s| s.sunset),
        coord: o.coord.as_ref().map(|c| Coord{ lat: c.lat, lon: c.lon }),
    }))
}


#[tokio::main]
async fn main() {
    // Reads RUST_LOG env var; defaults to "info"
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/weather", get(weather))
        .layer(CorsLayer::permissive()); // tighten this in production

    let addr = "0.0.0.0:8080";
    info!("Backend listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
