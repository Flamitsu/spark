#!/bin/bash
set -euo pipefail
# Check if running as root
# This final script will install the bootmanager into the VM enviroment (working on it)
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root (use doas/sudo)." 
   exit 1
fi
