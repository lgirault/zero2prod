#!/usr/bin/env bash

docker run --env-file zero2prod.env -p 8000:8000 zero2prod:latest
