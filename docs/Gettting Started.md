

Install with


```
pip install serve-rs
```

Navigate to a directory with a wsgi webapp, say django

```bash
(base) ashutoshpednekar@192 svc % ls | grep mana
manage.py
(base) ashutoshpednekar@192 svc % ls main/ | grep wsg
wsgi.py
(base) ashutoshpednekar@192 svc %
```

Run the server

```bash
(base) ashutoshpednekar@192 svc % serve-rs main.wsgi:application
[2025-02-22T06:16:50Z INFO  pubsub::common::nats::conn] stream updated successfully
WSGI Server running at http://127.0.0.1:8000
```

