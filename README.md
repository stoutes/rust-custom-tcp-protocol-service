This is a Rust service that uses a custom protocol to send and receive messages over a TCP connection.

Here's the protocol:

Bytes | Name | Description
--- | --- | ---
0-1 | **Magic Number** | Sending a magic number is a common way to ensure that the data you're receiving is what you expect.
2-3 | **Version Number** | start with version 1. We're going to use two bytes, so we have lots of room for future versions.
4-7 | **Timestamp** | a 32-bit unsigned integer to represent the number of seconds since the Unix epoch. This will give us a range of 1970-01-01 to 2106-02-07.
8-11 | **Payload size** | a 32-bit unsigned integer to represent the size of the payload.
12+ | **Payload** | JSON payload.
End-4 - End | **CRC32** | a CRC32 checksum to ensure that the data we received is the data we expected. Using the `crc32fast` crate to provide this functionality.


Just open two terminals in collector and server and do `cargo run` in each to see the action!

If you want to see the dashboard just hit `localhost:3000` in your browser.

There's also some REST endpoints you can hit:

  `api/all` - shows all collector data from the database
  
  `api/collectors` - shows all collectors in the pool
  
  `api/collector/{uuid}` - shows specific collector data based on provided uuid
  
  `api/collector/{uuid}/shutdown` - SHUT IT DOWN!
