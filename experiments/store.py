import os
from tqdm import tqdm

exp = (False, False)
mapping = "cyclical"

def growing(value: bool) -> str:
    return "growing" if value else "adult"

for i in tqdm(range(100)):
    cmd = "cargo run --release --quiet -- --nogui > "
    cmd += "brain-" + growing(exp[0])
    cmd += "_body-" + growing(exp[1])
    cmd += "/data/" + mapping + str(i)
    os.system(cmd)
