import requests
from bs4 import BeautifulSoup

def fetch_page_info(url):
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')

    title = soup.find('title').get_text() if soup.find('title') else 'No Title'
    meta_desc = soup.find('meta', attrs={'name': 'description'})
    description = meta_desc['content'] if meta_desc else 'No Description'

    return title, description

# usage example
url = 'https://www.example.com'
title, description = fetch_page_info(url)
print(f"Title: {title}\nDescription: {description}")
