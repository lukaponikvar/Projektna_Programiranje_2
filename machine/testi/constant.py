import requests

projects = requests.get("http://127.0.0.1:7878/project").json()
for projekt in projects:
    print(projekt, 3)
    if projekt["name"] == "Luka & Anara":
        url = "http://" + projekt["ip"] + ":" + \
            str(projekt["port"]) + "/sequence"
        print(url)
        sequences = requests.get(url).json()
        assert "Constant" in [zaporedje["name"] for zaporedje in sequences]
        k = 3
        for j in range(3):
            print(j)
            body = {
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [k],
                "sequences": []
            }
            print(j)
            r = requests.post(url + "/Constant", json=body)
            print(r.json())
            print("Hejhoj")
        print("ojla")
        break
        print("kaj")
    else:
        print("Luka & Anara not found")
        exit(1)

print("Hejhoj2")
quit()
