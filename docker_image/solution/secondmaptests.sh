#!/bin/bash

time ./m1_game_engine -f maps/map01 -p1 ./solution/target/release/filler -p2 m1_robots/h2_d2

time ./m1_game_engine -f maps/map01 -p2 ./solution/target/release/filler -p1 m1_robots/h2_d2

time ./m1_game_engine -f maps/map01 -p1 ./solution/target/release/filler -p2 m1_robots/h2_d2

time ./m1_game_engine -f maps/map01 -p2 ./solution/target/release/filler -p1 m1_robots/h2_d2

time ./m1_game_engine -f maps/map01 -p1 ./solution/target/release/filler -p2 m1_robots/h2_d2
