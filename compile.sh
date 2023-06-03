#!/bin/bash

cat ./measure/* >> summary.csv

sed -i 's/Time,Settings,Info,Wattage//g' summary.csv

sed -i '1s/^/Time,Settings,Info,Wattage\n/' summary.csv
