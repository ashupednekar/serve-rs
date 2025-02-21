use std::sync::Arc;

use hyper::{
   Body, Request, Response 
};
use pyo3::{prelude::*, types::IntoPyDict};

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
        println!("req: {:?}", &req);

        /*let headers: Vec<_> = req.headers()
            .iter()
            .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("")))
            .collect();*/

        let app = self.app.clone();

        let (status, response_headers, body) = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| -> PyResult<(String, Vec<(String, String)>, Vec<u8>)> {
                let environ = [
                    ("REQUEST_METHOD", req.method().as_str()),
                    ("PATH_INFO", req.uri().path()),
                    ("SERVER_PROTOCOL", "HTTP/1.1"),
                ]
                .into_py_dict(py)?;
                println!("prepared envs: {:?}", &environ);
                let res = app.call1(py, (environ,))?;
                println!("called python function");
                res.extract(py)
            })
        }).await.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))??;

        println!("{:?}| {:?} | {:?}", status, response_headers, body);
        
        let mut builder = Response::builder().status(status.parse::<u16>().unwrap_or(500));
        for (key, value) in response_headers {
            builder = builder.header(&key, &value);
        }
        
        Ok(builder.body(Body::from(body)).unwrap())
    }
}

