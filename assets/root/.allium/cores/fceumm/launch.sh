#!/bin/sh
cd /mnt/SDCARD/RetroArch/
HOME=/mnt/SDCARD/RetroArch/ ./retroarch -v -L .retroarch/cores/fceumm_libretro.so "$1"