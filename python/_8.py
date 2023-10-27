def read_file(path: str) -> str | None:
    try:
        with open(path) as file:
            out_put = file.read()
            return out_put
    except:
        return None



result = read_file("/mnt/d/Project/devfest/python/_1.py")

if result:
    print(f"{result}")
else:
    print("nothing")


