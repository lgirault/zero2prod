#!/usr/bin/env bash

tar -czf migrations.tgz migrations

scp migrations.tgz pi:.

rm migrations.tgz

