cmake_minimum_required(VERSION 3.15)
project(REPLispy C)

set(CMAKE_C_STANDARD 99)

add_executable(REPLispy main.c mpc.c mpc.h)