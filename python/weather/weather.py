import requests

def get_weather(city):
    api_key = "YOUR_API_KEY"
    url = f"http://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric"
    response = requests.get(url)
    data = response.json()
    return data

def main():
    print("Welcome to the Simple Weather App!")
    city = input("Enter city name: ")

    try:
        weather_data = get_weather(city)
        if weather_data["cod"] == 200:
            print("\nCurrent Weather in", city)
            print("Description:", weather_data["weather"][0]["desription"])
            print("Temperature:", weather_data["main"]["temp"], "°C")
            print("Humidity:", weather_data["main"]["humidity"], "%")
            print("Wind Speed:", weather_data["wind"]["speed"], "m/s")
        else:
            print("City not found. Please enter a valid city name.")
    except Exception as e:
        print("An error occurred:", e)

if __name__ == "__main__":
    main()
