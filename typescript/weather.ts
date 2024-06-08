#!/usr/bin/env node

import axios from 'axios';

async function getWeather(city: string): Promise<void> {
	const apiKey = 'YOUR_API_KEY';
	const url = `https://api.openweathermap.org/data/2.5/weather?q=${city}&appid=${apiKey}&units=metric`;

		try {
		const response = await axios.get(url);
		const { main, weather } = response.data;
		const temperature = main.temp;
		const weatherDescription = weather[0].description;

		console.log(`Current weather in ${city}:`);
		console.log(`Temperature: ${temperature}°C`);
		console.log(`Description ${weatherDescription}`);
	} catch (error) {
		console.error(`Error fetching weather data: ${error.message}`);
	}
}

function main(): void {
	const city = process.argv[2];
	if (!city) {
		console.log("Please provide a city name as an argument");
		return;
	}

	getWeather(city);
}

main();
