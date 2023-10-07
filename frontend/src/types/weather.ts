export interface WeatherData {
    latitude: number;
    longitude: number;
    generationtime_ms: number;
    utc_offset_seconds: number;
    timezone: string;
    timezone_abbreviation: string;
    elevation: number;
    current_weather_units: {
        time: string;
        temperature: string;
        windspeed: string;
        winddirection: string;
        is_day: string;
        weathercode: string;
    };
    current_weather_interval_seconds: number;
    current_weather: {
        time: string;
        temperature: number;
        windspeed: number;
        winddirection: number;
        is_day: number;
        weathercode: number;
    };
}
