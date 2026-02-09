
# rnd-rs

Extremely minimalist random bytes CLI. Because sometimes you need random bytes and the options are
    
    - openssl (ew)
    
    - piping /dev/urandom into some truncation black magic command
    
    - `python3 -c "print(__import__('os').urandom(32).hex())"`

This utility reduces this down to a simple `rnd hex -l 32`.

Features:
    
    - Encodings: base64, hex and raw bytes
    
    - Specifying length with -l/--length
    
    - Specifying randomness source (default is /dev/urandom), supply your own with -s/--source

Enjoy! :D

