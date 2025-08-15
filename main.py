from nspectre import py_scan_port, PortStatus 
from common_ports import MOST_COMMON_PORTS
import asyncio, argparse


async def _scan_wrapper(host: str, port: int):
    try:
        return await py_scan_port(host, port)
    except Exception as e:
        return e

async def main():

    single_port = False

    parser = argparse.ArgumentParser(
        prog="NSpectre",
        description="Simple port scanner",
        usage="%(prog)s HOSTNAME [options]"
    )
    parser.add_argument('hostname')
    parser.add_argument('-p', '--ports',
                        help='Specify the port to scan')

    args = parser.parse_args()
    host = args.hostname

    if args.ports is not None:
        ports_str: str = args.ports
        if '-' in ports_str:
            range_ends = ports_str.split('-', 1)
            i = int(range_ends[0])
            end = int(range_ends[1])
            ports = []
            while i < end:
                ports.append(i)
                i+=1
        elif ports_str.isnumeric():
            port = int(ports_str)
            ports = [port]
            single_port = True
        
            
    else:
        ports = MOST_COMMON_PORTS
    
    results = []

    async with asyncio.TaskGroup() as tg:
        for port in ports:
            t = tg.create_task(_scan_wrapper(host, port))
            results.append(t)

    for res in results:
        port = res.result()

        if single_port:
            print(f"{port}")
        else:
            if port.status==PortStatus.Open or port.status==PortStatus.Filtered:
                print(f"{port}")


asyncio.run(main())
