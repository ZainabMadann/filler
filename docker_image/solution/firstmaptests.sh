#!/bin/bash


time ./m1_game_engine -f maps/map00 -p1 ./solution/target/release/filler -p2 m1_robots/wall_e

time ./m1_game_engine -f maps/map00 -p2 ./solution/target/release/filler -p1 m1_robots/wall_e

time ./m1_game_engine -f maps/map00 -p1 ./solution/target/release/filler -p2 m1_robots/wall_e

time ./m1_game_engine -f maps/map00 -p2 ./solution/target/release/filler -p1 m1_robots/wall_e

time ./m1_game_engine -f maps/map00 -p2 ./solution/target/release/filler -p1 m1_robots/wall_e
