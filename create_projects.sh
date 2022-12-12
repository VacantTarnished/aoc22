#!/bin/bash

days=25
project_name="day_"

for ((i=8; i<=$days; i++)); do
    echo "Creating project $project_name$i"
    #cargo new $project_name$i
    #mkdir $project_name$i/input
    touch $project_name$i/input/input.txt
    echo "Done"
done
