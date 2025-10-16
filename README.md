# NSpectre

NSpectre is a simple network port scanner developed in Python and Rust. It leverages the speed of Rust for port scanning and the flexibility of Python for the user interface.

## Project Purpose

The purpose of Nspectre is to combine network programming with cybersec, while practicing and improve my coding skills and IT knowledge
## Features and Functionality

- **Fast Scanning:** The scanning core is implemented in Rust and uses the `tokio` library to perform asynchronous and parallel scans, ensuring quick results.
- **Three port states:** Detects whether a port is `Open`, `Closed`, or `Filtered` (in case of a timeout).
- **Flexibility in port selection:**
    - Scan a single port.
    - Scan a range of ports.
    - Scan a predefined list of the most common ports if no option is specified.
- **Clear Output:** Shows a list of `Open` and `Filtered` ports or the status of a single specified port.
- **Python/Rust Integration:** Uses `pyo3` and `maturin` to integrate Rust code into a Python application.

## Installation

To install and run the project, you need to have **Python** (version 3.8 or higher) and the **Rust toolchain** installed.

1.  **Clone the repository (if not already done):**
    ```bash
    git clone https://github.com/CiroBurro/NSpectre.git
    cd nspectre
    ```

2.  **Create a virtual environment (recommended):**
    ```bash
    python -m venv .venv
    source .venv/bin/activate  # On Windows: .venv\Scripts\activate
    ```

3.  **Install dependencies and compile the Rust module:**
    The `maturin` tool will automatically handle the compilation of the Rust code and the installation of the Python package.
    ```bash
    uv add maturin # Or pip install maturin
    maturin develop
    ```

## Usage

The program is used from the command line, specifying the hostname or IP address to scan and, optionally, the ports.

**Basic syntax:**

```bash
python main.py <HOSTNAME> [options]
```

### Arguments

- `HOSTNAME`: The IP address or hostname to scan (e.g., `192.168.1.1` or `google.com`).

### Options

- `-p, --ports`: Specifies the ports to scan. If not provided, the most common ports will be scanned.
    - **Single port:** `-p 80`
    - **Port range:** `-p 1-1024`

### Examples

- **Scan the most common ports of a host:**
    ```bash
    python main.py 192.168.1.1
    ```

- **Scan port 80:**
    ```bash
    python main.py example.com -p 80
    ```

- **Scan ports from 20 to 80:**
    ```bash
    python main.py 127.0.0.1 -p 20-80
    ```