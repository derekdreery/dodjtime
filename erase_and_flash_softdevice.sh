#!/bin/sh

# Use this if things aren't working before asking for help.
/opt/nrfjprog/nrfjprog --family NRF52 --program /opt/nrf5-sdk/components/softdevice/s112/hex/s112_nrf52_7.2.0_softdevice.hex --chiperase --verify
