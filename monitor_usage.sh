#!/bin/bash

LOGFOLDER="./logs"
LOGFILE="${LOGFOLDER}/usage_log_$(date "+%Y%m%d").txt"
INTERVAL=60  

mkdir -p "$LOGFOLDER"

echo "Monitoring system usage (CPU, Memory, GPU)" > $LOGFILE
echo "------------------------------------------" >> $LOGFILE

while true
do
  TIMESTAMP=$(date "+%Y-%m-%d %H:%M:%S")

  CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2 + $4 + $6}')

  MEM_TOTAL=$(free | grep Mem | awk '{print $2}')
  MEM_USED=$(free | grep Mem | awk '{print $3}')
  MEM_USAGE=$(echo "scale=2; $MEM_USED/$MEM_TOTAL*100" | bc)

  if command -v nvidia-smi &> /dev/null
  then
    GPU_USAGE=$(nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits)
    GPU_MEM_USAGE=$(nvidia-smi --query-gpu=utilization.memory --format=csv,noheader,nounits)
  else
    GPU_USAGE="N/A"
    GPU_MEM_USAGE="N/A"
  fi

  echo "Timestamp: $TIMESTAMP" \
  "CPU Usage: $CPU_USAGE%" \
  "Memory Usage: $MEM_USAGE%" \
  "GPU Usage: $GPU_USAGE%" \
  "GPU Memory Usage: $GPU_MEM_USAGE%"  >> $LOGFILE

  sleep $INTERVAL
done

