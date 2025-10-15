from nspectre import py_scan_port, PortStatus 
from utils import *
import asyncio, time


async def _scan_wrapper(host: str, port: int):
    try:
        return await py_scan_port(host, port)
    except Exception as e:
        return e

def print_ports(results, single_port):

    for res in results:
        port = res.result()

        if single_port:
            print(f"{port}")
        else:
            if port.status==PortStatus.Open or port.status==PortStatus.Filtered:
                print(f"{port}")


async def main():
    start = time.time()
    single_port = False

    arg = parser.parse_args()
    host = arg.hostname
    ports = ports_selection(arg, single_port)

    results = []

    async with asyncio.TaskGroup() as tg:
        for port in ports:
            t = tg.create_task(_scan_wrapper(host, port))
            results.append(t)


    print_ports(results, single_port)

    end = time.time()
    print(f"Scan lasted {round(end -start, 2)} seconds")

asyncio.run(main())
