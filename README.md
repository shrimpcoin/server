# Shrimpcoin API backend
The backend for https://github.com/shrimpcoin/site and the API for the Shrimpcoin assembler.

This project is meant to be run as a submodule of the [Shrimpcoin website](https://github.com/shrimpcoin/site); however, it is possible to run it standalone by not using the ``BUILD_PATH`` environment variable. This will disable the webserver functionality and only use the assembler API.

## Running
This project is meant to be run with ``yarn serve`` in the parent directory, but it can also be run with ``cargo run``.
