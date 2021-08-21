filename = "README.md"
url = "https://github.com/Ismail-Maj/LeetCode-Rust/blob/main"

with open(filename, "r+") as f:
    data = f.read().split('\n')
    f.seek(0)
    folder = ""
    idx = 0
    for i, line in enumerate(data):
        if line[:2] == "##":
            folder = line.split()[-1]
        elif line[:6] == '- [ ] ':
            problem = line.split('/')[-2]
            link = line.split('(')[-1][:-1]
            new_line = '- [x] '
            new_line += " ".join(list(map(lambda s: s[0].upper() + s[1:], problem.split('-'))))
            new_line += f" [Problem]({link}) [Code]({url}/{folder}/{problem}.rs)"
            idx = i
            break

    data[idx] = new_line
    f.truncate(0)
    f.write("\n".join(data))
    f.truncate()

with open(f'{folder}/{problem}.rs', 'x') as f:
    f.write(link +'\n\n'+'struct Solution\n\n')

import webbrowser

webbrowser.open(link)

