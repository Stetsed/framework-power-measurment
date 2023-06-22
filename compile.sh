#!/bin/bash

cat ./measure/* >>summary_pre.csv

sed -i 's/Time,Settings,Info,Wattage//g' summary_pre.csv

echo "Time,Settings,Info,Wattage" >summary.csv

cat summary_pre.csv >>summary.csv

rm summary_pre.csv
