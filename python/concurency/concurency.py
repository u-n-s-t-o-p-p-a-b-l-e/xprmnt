import aiohttp
import asyncio

async def fetch(url):
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            return await response.text()

async def main(urls):
    tasks =  [fetch(url) for url in urls]
    responses = await asyncio.gather(*tasks)
    for response in responses:
        print(response[:100])

# Usage example
urls = ['https://www.example.com', 'https://www.python.org', 'https://www.github.com']
asyncio.run(main(urls))
