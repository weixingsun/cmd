#!/bin/bash

#tune(){
#  NAME=$1
#  CPU=$2
#  /root/tune_irq_arm --mode=set --name=$NAME --cpu=$CPU --static
#}

#array_def_cpus=( 44  45 108 109  46  47 110 111  62  63 126 127  172 173 236 237 174 175 238 239 188 189 252 253 )

#tune_host(){
#  array_pci_host=($(lspci -d:200 |awk '{print $1}'))
#  for i in "${!array_cpu_host[@]}"; do
#    tune ${array_pci_host[i]}  ${array_def_cpu_host[i]} 2>/dev/null |grep -v title
#  done
#}


ACT=${1:-set}  # get/set
IDX=${2:-9999}

tune(){
  NAME=$1
  CPU=$2
  #echo "./tune_irq_arm --mode=$ACT --name=$NAME --cpu=$CPU --static --index=$IDX"
  /root/tune_irq_arm --mode=$ACT --name=$NAME --cpu=$CPU --static --index=$IDX
}

#array_def_cpus=( 44  45 108 109  46  47 110 111  62  63 126 127  172 173 236 237 174 175 238 239 188 189 252 253 )
array_def_cpus=( 124 124 124 124 124 124 125 125 125 125 125 125  254 254 254 254 254 254 255 255 255 255 255 255 )
array_idx_cpus=(  44  45 108 109  46  47 110 111  62  63 126 127  172 173 236 237 174 175 238 239 188 189 252 253 )

tune_host(){
  array_pci_host=($(lspci -d:200 |awk '{print $1}'))
  for i in "${!array_def_cpus[@]}"; do
    tune ${array_pci_host[i]}  ${array_def_cpus[i]} 2>/dev/null |grep -v title
  done
}
tune_host_idx(){
  array_pci_host=($(lspci -d:200 |awk '{print $1}'))
  for i in "${!array_idx_cpus[@]}"; do
    tune ${array_pci_host[i]}  ${array_idx_cpus[i]} 2>/dev/null |grep -v title
  done
}

if [ $IDX -lt 9999 ]; then
  tune_host
else
  tune_host_idx
fi
