import argparse
from utils.common_ports import MOST_COMMON_PORTS

parser = argparse.ArgumentParser(
    prog="NSpectre",
    description="Simple port scanner",
    usage="%(prog)s HOSTNAME [options]"
)
parser.add_argument('hostname')
parser.add_argument('-p', '--ports',
                    help='Specify the port to scan')

def ports_selection(args, single_port):
    ports = []

    if args.ports is not None:
        ports_str: str = args.ports
        if '-' in ports_str:
            range_ends = ports_str.split('-', 1)
            i = int(range_ends[0])
            end = int(range_ends[1])


            while i < end:
                ports.append(i)
                i+=1

        elif ports_str.isnumeric():
            port = int(ports_str)
            ports = [port]
            single_port = True

        return ports
    else:
        return MOST_COMMON_PORTS
