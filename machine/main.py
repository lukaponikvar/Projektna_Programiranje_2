import requests

projects = requests.get("http://127.0.0.1:7878/project").json()
for projekt in projects:
    if projekt["name"] == "Matija & Filip":
        url = "http://" + projekt["ip"] + ":" + \
            str(projekt["port"]) + "/sequence"
        print(url)
        sequences = requests.get(url).json()
        assert "Arithmetic" in [zaporedje["name"] for zaporedje in sequences]
        k = 10
        z = 0
        for j in range(100):
            body = {
                "range": {
                    "from": j * 100,
                    "to": (j + 1) * 100,
                    "step": 1,
                },
                "parameters": [z, k],
                "sequences": [
                    {
                        "name": "Arithmetic",
                        "parameters": [z, k],
                        "sequences": [
                            {
                                "name": "Arithmetic",
                                "parameters": [z, k],
                                "sequences": [],
                            }
                        ],
                    },
                ],
            }
            r = requests.post(url + "/Arithmetic", json=body)
            # print(r)
            print(r.json())
        break
    else:
        print("Matija & Filip not found")
        exit(1)
