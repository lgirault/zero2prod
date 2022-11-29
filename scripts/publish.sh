#!/usr/bin/env bash
set -x
set -eo pipefail


docker build --tag zero2prod --file Dockerfile .

time=`date +"%Y-%m-%d_%Hh%M"`
name_tag=lgirault/my-repo:zero2prod-snapshot-$time
docker tag zero2prod:latest $name_tag
docker push $name_tag
