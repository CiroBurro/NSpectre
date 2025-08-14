use pyo3::prelude::*;
use tokio::{net::TcpStream, runtime::Builder};

#[pyclass(get_all, set_all)]
#[derive(Debug)]
struct Port {
    pub port: u16,
    pub status: PortStatus,
}

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
enum PortStatus {
    Open,
    Closed,
    Filtered,
}

async fn scan_port(host: &str, port: u16) -> Port {
    let socket_addr = format!("{host}:{port}");
    let status = match TcpStream::connect(&socket_addr).await {
        Ok(_) => PortStatus::Open,
        Err(_) => PortStatus::Closed,
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
    let mut builder = Builder::new_multi_thread();
    builder.enable_io();
    let builder = builder;
    pyo3_asyncio::tokio::init(builder);
    m.add_function(wrap_pyfunction!(py_scan_port, m)?)?;
    m.add_class::<Port>()?;
    m.add_class::<PortStatus>()?;
    Ok(())
}
