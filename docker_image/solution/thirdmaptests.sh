#!/bin/bash

time ./m1_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 m1_robots/bender

time ./m1_game_engine -f maps/map02 -p2 ./solution/target/release/filler -p1 m1_robots/bender

time ./m1_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 m1_robots/bender

time ./m1_game_engine -f maps/map02 -p2 ./solution/target/release/filler -p1 m1_robots/bender

time ./m1_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 m1_robots/bender