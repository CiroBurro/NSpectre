use pyo3::prelude::*;
use std::fmt;
use tokio::{
    net::TcpStream,
    runtime::Builder,
    time::{timeout, Duration},
};

/// Port Structure
/// Represents a socket port
///
/// Fields:
/// - port: actual number of the port
/// - status: status of the port (open, closed or filtered)
#[pyclass(get_all, set_all)]
#[derive(Debug)]
struct Port {
    pub port: u16,
    pub status: PortStatus,
}

#[pymethods]
impl Port {
    /// Representing method for Port
    fn __repr__(&self) -> String {
        format!("Port: {} - Status: {}", self.port, self.status)
    }
}

/// PortStatus Enum
/// Represents the status of a port
///
/// Variants:
/// - Open: the port is open
/// - Closed: the port is closed
/// - Filtered: the port is filtered
#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
enum PortStatus {
    Open,
    Closed,
    Filtered,
}

#[pymethods]
impl PortStatus {
    /// Representing method for PortStatus
    fn __repr__(&self) -> String {
        match self {
            PortStatus::Open => String::from("Open"),
            PortStatus::Closed => String::from("Closed"),
            PortStatus::Filtered => String::from("Filtered"),
        }
    }
}

// Display trait implementation for PortStatus
impl fmt::Display for PortStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortStatus::Open => write!(f, "open"),
            PortStatus::Closed => write!(f, "closed"),
            PortStatus::Filtered => write!(f, "filtered"),
        }
    }
}

/// Scan port function
/// Checks the status of the given port of the given host
///
/// Args:
/// - host: ip address of the host to scan
/// - port: port to scan
///
/// Returns: port scanned
async fn scan_port(host: &str, port: u16) -> Port {
    let socket_addr = format!("{host}:{port}");
    let status = match timeout(Duration::from_secs(2), TcpStream::connect(&socket_addr)).await {
        Ok(Ok(_)) => PortStatus::Open,
        Ok(Err(_)) => PortStatus::Closed,
        Err(_) => PortStatus::Filtered,
    };

    Port { port, status }
}

/// Py scan port function
/// Scan port function wrapper for asyncio in python
///
/// Args:
/// - host: ip address of the host to scan
/// - port: port to scan
///
/// Returns: port scanned within a result wrapper
#[pyfunction]
fn py_scan_port<'p>(py: Python<'p>, host: String, port: u16) -> PyResult<&'p PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let port = scan_port(host.as_str(), port).await;
        Ok(port)
    })
}


/// Nspectre python module
/// Initializes the tokio async runtime and adds py_scan_port function, Port struct and PortStatus enum to the module
#[pymodule]
fn nspectre(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let mut builder = Builder::new_multi_thread();
    builder.enable_all();
    let builder = builder;
    pyo3_asyncio::tokio::init(builder);
    
    
    m.add_function(wrap_pyfunction!(py_scan_port, m)?)?;
    m.add_class::<Port>()?;
    m.add_class::<PortStatus>()?;
    Ok(())
}
