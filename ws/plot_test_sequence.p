#!/usr/bin/env gnuplot

set terminal pdf size 10,1
set output "test_sequence.pdf"
# set xrange [-5:]
set yrange [-1:2]
unset key
set border 3

set xtics nomirror
set ytics nomirror
set xtics 1

set grid x

# set tics font "Helvetica,50"
set style arrow 1 nohead linewidth 3
set datafile separator ','
plot '${PYROLYSE_PATH}/test_data/interval_all_wo_header.csv' using 3 : 5 : ($4-$3+1) : (0.0) with vector as 1, \
     '${PYROLYSE_PATH}/test_data/interval_all_wo_header.csv' using 3 : 5 : 2 with labels left offset 0.1,0.5
     
     
