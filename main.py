from nspectre import py_scan_port, PortStatus 
from utils import *
import asyncio, time


async def _scan_wrapper(host: str, port: int):
    """
    Wrapper to py_scan_port function, needed to allow python to interpret that function as async
    :param host: ip address of the host to scan
    :param port: port to scan
    :return: a future with the scan result
    """
    try:
        return await py_scan_port(host, port)
    except Exception as e:
        return e

def print_ports(results, single_port):
    """
    Prints the result of the scan. It prints only the open and filtered ports if a range of ports was specified, instead if only a port was scanned it prints its status
    :param results: list with the results of the scan
    :param single_port: single_port flag specifies if only a single port was scanned
    :return: Null
    """
    for res in results:
        port = res.result()

        if single_port:
            print(f"{port}")
        else:
            if port.status==PortStatus.Open or port.status==PortStatus.Filtered:
                print(f"{port}")


async def main():
    """
    Main function of the program: collects the arguments, creates an async task to scan every port in parallel and finally prints the results
    :return: Null
    """
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
