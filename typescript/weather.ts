#!/usr/bin/env node

import axios from 'axios';

async function getWeather(city: string): Promise<void> {
	const apiKey = 'YOUR_API_KEY';
	const url = `https://api.openweathermap.org/data/2.5/weather?q=${city}&appid=${apiKey}&units=metric`;

}
