import asyncio
import time

import aiohttp
import requests

NUM_REQUESTS = 100
bad = False
async_bad = False


async def get_response(session: aiohttp.ClientSession):
    # async with session.post("https://www.example.com", data=b"request body", headers={"header-name": "value"}, timeout=aiohttp.ClientTimeout(total=1)) as response:
    async with session.get("https://www.example.com", timeout=aiohttp.ClientTimeout(total=1)) as response:
        if response.status != 200:
            global async_bad
            async_bad = True
        # else:
        #     content_type = response.headers["Content-Type"]
        #     print(content_type)
        #     body = await response.text()
        #     print(body)


async def async_main():
    async with aiohttp.ClientSession() as session:
        tasks = [asyncio.create_task(get_response(session)) for _ in range(NUM_REQUESTS)]
        await asyncio.gather(*tasks)


def main():
    for _ in range(NUM_REQUESTS):
        # response = requests.post("https://www.example.com", data=b"request body", headers={"header-name": "value"}, timeout=1)
        response = requests.get("https://www.example.com", timeout=1)
        if response.status_code != 200:
            global bad
            bad = True
        # else:
        #     content_type = response.headers["Content-Type"]
        #     print(content_type)
        #     body = response.text
        #     print(body)


start_time = time.time()
asyncio.run(async_main())
end_time = time.time()
elapsed_time = round(end_time - start_time, 3)
if async_bad:
    print("Had failed asynchronous request")
else:
    print(f"Asynchronous elapsed time for {NUM_REQUESTS} requests = {elapsed_time} seconds")

start_time = time.time()
main()
end_time = time.time()
elapsed_time = round(end_time - start_time, 3)
if bad:
    print("Had failed synchronous request")
else:
    print(f" Synchronous elapsed time for {NUM_REQUESTS} requests = {elapsed_time} seconds")
