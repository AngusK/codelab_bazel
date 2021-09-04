#!/bin/bash
pip-compile -r --allow-unsafe --output-file=requirements_lock.txt requirements.in
