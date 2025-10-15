use pyo3::prelude::*;
use std::fmt;
use tokio::{
    net::TcpStream,
    runtime::Builder,
    time::{timeout, Duration},
};

#[pyclass(get_all, set_all)]
#[derive(Debug)]
struct Port {
    pub port: u16,
    pub status: PortStatus,
}

#[pymethods]
impl Port {
    fn __repr__(&self) -> String {
        format!("Port: {} - Status: {}", self.port, self.status)
    }
}

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
enum PortStatus {
    Open,
    Closed,
    Filtered,
}

#[pymethods]
impl PortStatus {
    fn __repr__(&self) -> String {
        match self {
            PortStatus::Open => String::from("Open"),
            PortStatus::Closed => String::from("Closed"),
            PortStatus::Filtered => String::from("Filtered"),
        }
    }
}

impl fmt::Display for PortStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortStatus::Open => write!(f, "open"),
            PortStatus::Closed => write!(f, "closed"),
            PortStatus::Filtered => write!(f, "filtered"),
        }
    }
}

async fn scan_port(host: &str, port: u16) -> Port {
    let socket_addr = format!("{host}:{port}");
    let status = match timeout(Duration::from_secs(2), TcpStream::connect(&socket_addr)).await {
        Ok(Ok(_)) => PortStatus::Open,
        Ok(Err(_)) => PortStatus::Closed,
        Err(_) => PortStatus::Filtered,
    };

    Port { port, status }
}

#[pyfunction]
fn py_scan_port<'p>(py: Python<'p>, host: String, port: u16) -> PyResult<&'p PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let port = scan_port(host.as_str(), port).await;
        Ok(port)
    })
}

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
