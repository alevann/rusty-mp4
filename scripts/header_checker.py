def do_add(s, x):
    l = len(s)
    s.add(x)
    return len(s) != l

b = ""
with open("./scripts/header_positions.txt", "r") as f:
    b = f.readlines()

b[0] = b[0].split(', ')
#b[1] = b[1].split(', ')

for i, l in enumerate(b):
    d = set()
    s = set()
    for n in l:
        if not do_add(s, int(n)):
            d.add(n)
    print(f"Duplicates: {[n for n in d]}")
