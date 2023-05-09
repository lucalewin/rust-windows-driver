# Rusty Windows Driver

Basic template for a windows driver written in Rust.

## How to run

Register the driver:
`sc create DRIVER_NAME binPath="ABSOLUTE_PATH_TO_DRIVER" type=kernel`

Start the driver:
`sc start DRIVER_NAME`

Stop the driver:
`sc stop DRIVER_NAME`

Unregister the driver:
`sc delete DRIVER_NAME`
