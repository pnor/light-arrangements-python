# Light Arrangements : Python
Bindings for the light arrangements library in python

See the original repo here:
https://github.com/pnor/light-arrangements

## Running Tests

``` sh
# Enter python virtual environemnt; make one if not made already
source venv/bin/activate
maturin build
pip install .
pytest
```

## On Implementation
PyO3 requires classes to be Send, which is something neither the rs_ws281x library or TestStrip visual backend implement. To get around this, I construct these objects on a seperate thread and have the python objects communicate via async channels.
