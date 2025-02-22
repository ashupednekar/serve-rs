# serve-rs: Finally, A Gunicorn Replacement That Doesn’t Make You Cry  

## Installation  

Why wrestle with Gunicorn’s infinite config options when you can just **install serve-rs** and move on with your life?  

```bash  
pip install serve-rs  
```  

## Running Your WSGI App (Without Losing Your Sanity)  

So you have a WSGI web app. Maybe Django, maybe Flask, maybe something cursed.  
Navigate to your project directory and check for a WSGI entry point.  

```bash  
(base) ashutoshpednekar@192 svc % ls | grep mana  
manage.py  
(base) ashutoshpednekar@192 svc % ls main/ | grep wsg  
wsgi.py  
```  

Now, unlike Gunicorn, where you have to decide between **sync, async, gevent, eventlet, tornado, uvicorn workers**  
(seriously, why are there so many worker models?), **serve-rs** just *works*:  

```bash  
(base) ashutoshpednekar@192 svc % serve-rs main.wsgi:application  
[2025-02-22T06:16:50Z INFO  pubsub::common::nats::conn] stream updated successfully  
WSGI Server running at http://127.0.0.1:8000  
```  

No `--workers`, no `--preload`, no `"what even is a worker threadpool in 2025?"`—just **run your server(s)**.  
(Yes, that pun was intentional.)  

## Test It Like You Mean It  

Use `curl`, because real devs test APIs from the terminal:  

```bash  
ashu@ashu:~ $ curl http://localhost:8000/screenmgmt/screen/  
{"errors":[{"code":"ER-0014","detail":"Project is not selected. Please select the project to continue.","attr":null}]}  
```  

## Why You’ll Never Look at Gunicorn Again  

- **No more million worker models** – You don’t have to Google *"Gunicorn workers explained like I'm five"*.  
- **Processes? In 2025?** – In the age of **containers and serverless**, why are you still micromanaging worker PIDs?  
- **Rust-powered speed** – Because Python’s GIL has ruined enough of your life already.  
- **Dead simple deployment** – You don’t need an entire config file just to start your server.  

### TL;DR: Run `serve-rs` and move on with your life.  
