#!/bin/bash

days=25
project_name="day_"

for ((i=2; i<=$days; i++)); do
    echo "Creating project $project_name$i"
    cargo new $project_name$i
    echo "Done"
done
