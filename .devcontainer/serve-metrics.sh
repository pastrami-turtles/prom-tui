#!/bin/sh
METRICS_FILE=/home/metrics.http
while true; do cat $METRICS_FILE | nc -l -q 1 -p 8080; done