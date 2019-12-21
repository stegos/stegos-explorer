#!/bin/env bash

do_func1() {
  echo Hi from func1
}


do_func2() {
  echo Hi from func2 with argument $1
}




case $1 in
func1 | func2  )
    do_$1 $2
    ;;
*)
    echo 2>&1 Use func1 or func2 as argument
    exit 1
    ;;
esac


