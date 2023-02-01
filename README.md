# PNG_in_Rust

Explore the format of PNG by Rust

## Introduction

Sample png file in this project: `./png/test.png`

```console
cd ./png
$ cargo run
File Name: test.png
Signature[bytes]:
[137, 80, 78, 71, 13, 10, 26, 10]
Chunk size: 13
Chunk type: "IHDR" (0x49484452)
Chunk crc: 0x9A768270
---------------
Chunk size: 226876
Chunk type: "IDAT" (0x49444154)
Chunk crc: 0x177D762A
---------------
Chunk size: 0
Chunk type: "IEND" (0x49454e44)
Chunk crc: 0xAE426082
---------------
***End***
```

## References

* [PNG spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
* [Hiding information inside of PNG](https://www.youtube.com/watch?v=M9ZwuIv3xz8&t=2285s&ab_channel=TsodingDaily)
