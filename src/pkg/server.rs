use hyper::{
    service::{make_service_fn, service_fn}, Server
};
use pyo3::prelude::*;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use crate::pkg::wsgi::WSGIApp;

pub async fn serve() -> PyResult<()>{
    let wsgi_module = "main.wsgi";  
    let wsgi_app = "application";
    
    let app = Arc::new(Python::with_gil(|py|{
        WSGIApp::new(py, wsgi_module, wsgi_app)
    })?);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let make_svc = make_service_fn(move |_| {
        let app = app.clone();
        async { 
            Ok::<_, Infallible>(service_fn(move |req| {
                let app = app.clone();
                async move { app.handle_request(req).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    
    println!("WSGI Server running at http://{}", addr);
    server.await.unwrap();
    Ok(())
}
