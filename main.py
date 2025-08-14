from nspectre import py_scan_port 
import asyncio

async def main():
    host = "192.168.8.172"
    port = 90
    result = await py_scan_port(host, port)
    print(result.port)


asyncio.run(main())
