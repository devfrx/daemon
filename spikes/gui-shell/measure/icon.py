"""SP-8 -- a square PNG for `tauri icon`, with the standard library only. Usage: python icon.py <out.png> [size]"""
import struct
import sys
import zlib


def chunk(kind: bytes, data: bytes) -> bytes:
    return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)


def png(path: str, size: int, rgb: tuple) -> None:
    row = b"\x00" + bytes(rgb) * size
    raw = row * size
    data = b"\x89PNG\r\n\x1a\n"
    data += chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 2, 0, 0, 0))
    data += chunk(b"IDAT", zlib.compress(raw, 9))
    data += chunk(b"IEND", b"")
    with open(path, "wb") as f:
        f.write(data)


if __name__ == "__main__":
    png(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 1024, (30, 60, 110))
