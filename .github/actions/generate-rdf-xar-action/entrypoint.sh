#!/bin/sh -l

echo "Hello $1"

ls -l .

time=$(date)
echo "time=$time" >> $GITHUB_OUTPUT
