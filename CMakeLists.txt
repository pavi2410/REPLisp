cmake_minimum_required(VERSION 3.15)
project(REPLisp C)

set(CMAKE_C_STANDARD 23)

add_executable(REPLisp main.c mpc.c mpc.h)