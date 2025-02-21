use std::{collections::HashMap, sync::Arc};

use hyper::{
   Body, Request, Response 
};
use pyo3::{prelude::*, types::{PyBytes, PyDict}};

use crate::pkg::wsgi::response::WsgiResponse;

use super::WSGIApp;


impl WSGIApp{

    pub async fn handle_request(&self, req: Request<Body>) -> PyResult<Response<Body>> {
        tracing::info!("req: {:?}", &req);

        let path = req.uri().to_string();
        let headers: HashMap<String, String> = req.headers()
            .iter()
            .map(|(k, v)| {
                let key = format!("HTTP_{}", k.as_str().replace("-", "_").to_uppercase());
                let value = v.to_str().unwrap_or("").to_string();
                (key, value)
            })
            .collect();
        let body_bytes = hyper::body::to_bytes(req.into_body())
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?
            .to_vec();


        let app = self.app.clone();

        let (status, response_headers, body) = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| -> PyResult<(String, Vec<(String, String)>, Vec<u8>)> {
                let environ = PyDict::new(py);
                for (k, v) in headers.into_iter(){
                    environ.set_item(k.as_str().replace("-", "_").to_uppercase(), v.to_string())?;
                }
                environ.set_item("SERVER_NAME", "")?;
                environ.set_item("SERVER_PORT", "")?;
                environ.set_item("HTTP_HOST", "localhost")?;
                environ.set_item("PATH_INFO", path)?;
                environ.set_item("REQUEST_METHOD", "GET")?;

                let py_body = PyBytes::new(py, &body_bytes);

                let io = py.import("io")?;
                let wsgi_input = io.getattr("BytesIO")?.call1((py_body,))?;
                environ.set_item("wsgi.input", wsgi_input)?;

                environ.set_item("wsgi.version", (1, 0))?;
                environ.set_item("wsgi.errors", py.None())?;

                tracing::debug!("prepared environ: {:?}", environ);

                let wsgi_response = Py::new(py, WsgiResponse::new())?;
                let start_response = wsgi_response.getattr(py, "start_response")?;
                let res = app.call1(py, (environ, start_response, ))?;
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

