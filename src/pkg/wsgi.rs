use std::{collections::HashMap, sync::Arc};

use hyper::{
   Body, Request, Response 
};
use pyo3::{prelude::*, types::{IntoPyDict, PyBytes, PyDict}};

#[pyfunction]
fn start_response(){}


pub struct WSGIApp {
    app: Arc<Py<PyAny>>,
}

impl WSGIApp {
    pub fn new(py: Python, module: &str, app: &str) -> PyResult<Self> {
        let module = py.import(module)?;
        let app = Arc::new(module.getattr(app)?.into_pyobject(py)?.into());
        Ok(Self { app })
    }

    pub async fn handle_request(&self, req: Request<Body>) -> PyResult<Response<Body>> {
        tracing::info!("req: {:?}", &req);

        /*let headers: Vec<_> = req.headers()
            .iter()
            .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("")))
            .collect();*/

        let body_bytes = hyper::body::to_bytes(req.into_body())
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?
            .to_vec();

        let app = self.app.clone();

        let (status, response_headers, body) = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| -> PyResult<(String, Vec<(String, String)>, Vec<u8>)> {
                let environ = PyDict::new(py);

                environ.set_item("SERVER_NAME", "")?;
                environ.set_item("SERVER_PORT", "")?;
                environ.set_item("HTTP_HOST", "localhost")?;

                environ.set_item("REQUEST_METHOD", "GET")?;



                let py_body = PyBytes::new(py, &body_bytes);

                let io = py.import("io")?;
                let wsgi_input = io.getattr("BytesIO")?.call1((py_body,))?;
                environ.set_item("wsgi.input", wsgi_input)?;

                environ.set_item("wsgi.version", (1, 0))?;
                environ.set_item("wsgi.errors", py.None())?;

                tracing::debug!("prepared environ: {:?}", environ);

                let headers = PyDict::new(py);

                let res = app.call1(py, (environ, start_response(), ))?;
                tracing::info!("called Python WSGI function");

                let response_body: Vec<u8> = res.extract(py)?;
                //let status_str: String = status_code.extract(py)?;
                //let response_headers: Vec<(String, String)> = headers.extract(py)?;

                Ok(("200".to_string(), vec![("".to_string(), "".to_string())], response_body))   
            })
        }).await.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))??;

        tracing::info!("{:?}| {:?} | {:?}", status, response_headers, body);
        
        let mut builder = Response::builder().status(status.parse::<u16>().unwrap_or(500));
        for (key, value) in response_headers {
            builder = builder.header(&key, &value);
        }
        
        Ok(builder.body(Body::from(body)).unwrap())
    }
}

