import requests

projects = requests.get("http://127.0.0.1:7878/project").json()
for projekt in projects:
    print(projekt, 3)
    if projekt["name"] == "Luka & Anara":
        url = "http://" + projekt["ip"] + ":" + \
            str(projekt["port"]) + "/sequence"
        print(url)
        sequences = requests.get(url).json()
        assert "Sum" in [zaporedje["name"] for zaporedje in sequences]
        body = {
            "range": {
                "from": 0,
                "to": 10,
                "step": 1,
            },
            "parameters": [],
            "sequences": [
                {
                    "name": "Arithmetic",
                    "parameters": [3, 2],
                    "sequences": [],
                },
                {
                    "name": "Arithmetic",
                    "parameters": [4, 2],
                    "sequences": [],
                },
            ],
        }
        r = requests.post(url + "/Sum", json=body)
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
