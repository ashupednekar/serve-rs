from servers import start
from pathlib import Path


def serve():
    cwd: str = Path.cwd()
    print(f"cwd: {cwd}")
    start()
