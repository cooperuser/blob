from typing import List, Tuple
import pexpect
import re

BIN = "./target/release/blob"
PATTERN = "b'Vector { x: ([-\\d.]+), y: ([-\\d.]+) }'"

def get_points(output: bytes | str) -> List[Tuple[float, float]]:
    points = []
    for line in output.splitlines():
        m = re.match(PATTERN, str(line))
        if not m: continue
        g = m.groups()
        points.append((float(g[0]), float(g[1])))
    return points

def get_output(proc) -> bytes:
    proc.sendline("")
    proc.read_nonblocking(10)
    return proc.read_nonblocking(10000)

def get_proc() -> pexpect.spawn:
    return pexpect.spawn(BIN)

if __name__ == "__main__":
    proc = get_proc()

    for i in range(2):
        output = get_output(proc)
        print(get_points(output))

    proc.close()
