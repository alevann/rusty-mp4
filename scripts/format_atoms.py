# Formats the atom identifiers as an array to be copy pasted in code

buff = ["["]

with open("./a.txt", "r") as f:
    for l in f.readlines():
        buff.append('"' + l.split()[0] + '",')

buff.append("]")

with open("./o.txt", "w") as f:
    f.write(' '.join(buff))