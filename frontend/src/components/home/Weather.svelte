<script lang="ts">
    import axios from "axios";
    import type {WeatherData} from "@src/types/weather.ts";

    function getLocation(): Promise<GeolocationPosition> {
        return new Promise((resolve, reject) => {
            navigator.geolocation.getCurrentPosition(resolve, reject);
        });
    }

    interface Location {
        latitude: number;
        longitude: number;
    }

    let location: Promise<Location> = (async () => {
        let latitude;
        let longitude;

        try {
            const position = await getLocation();

            latitude = position.coords.latitude;
            longitude = position.coords.longitude;
        } catch {
            latitude = 51.5085;
            longitude = -0.1257;
        }

        return {
            latitude,
            longitude,
        };
    })();

    let url = (async () => {
        return `https://api.open-meteo.com/v1/forecast?latitude=${(await location).latitude}&longitude=${(await location).longitude}&current_weather=true&forecast_days=1`;
    })();

    let weather = (async () => {
        try {
            return (
                await axios
                    .get<WeatherData>(
                        await url,
                        {
                            timeout: 60000,
                        },
                    )
            ).data;
        } catch (e) {
            console.error(e);
            throw e;
        }
    })();
</script>

{#await url then url}
    <a href={url} target="_blank">
        {#await weather}
            <p class="font-mono text-sm">Loading Weather...</p>
        {:then weather}
            <p class="font-mono text-sm">{`${weather.current_weather.temperature} ${weather.current_weather_units.temperature}, ${weather.current_weather.windspeed} ${weather.current_weather_units.windspeed}`}</p>
        {/await}
    </a>
{/await}
