#!/bin/bash
ACT=${1:-set}  # get/set
IDX=${2:-9999}

tune(){
  NAME=$1
  CPU=$2
  /root/tune_irq_arm --mode=$ACT --name=$NAME --cpu=$CPU --static --index=$IDX
}

#array_def_guest=( 124 124 124 124   124 124 125 125   125 125 125 125 )
#array_cpu_guest=(  44  45 108 109    46  47 110 111    62  63 126 127 )
array_def_guest=( 126 126 126 126   126 126 127 127   127 127 127 127 )
array_idx_guest=(  44  45 108 109    46  47 110 111    60  61 124 125 )

tune_guest(){
  array_pci0=($(lspci -d:200 |awk '{print $1}'))
  for i in "${!array_def_guest[@]}"; do
    tune ${array_pci0[i]}  ${array_def_guest[i]}
  done
}
tune_guest_idx(){
  array_pci0=($(lspci -d:200 |awk '{print $1}'))
  for i in "${!array_idx_guest[@]}"; do
    tune ${array_pci0[i]}  ${array_idx_guest[i]}
  done
}

if [ $IDX -lt 9999 ]; then
  tune_guest
else
  tune_guest_idx
fi
