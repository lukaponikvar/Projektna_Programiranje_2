import requests

projects = requests.get("http://127.0.0.1:7878/project").json()
for projekt in projects:
    print(projekt, 3)
    if projekt["name"] == "Luka & Anara":
        url = "http://" + projekt["ip"] + ":" + \
            str(projekt["port"]) + "/sequence"
        print(url)
        sequences = requests.get(url).json()
        assert "Arithmetic" in [zaporedje["name"] for zaporedje in sequences]
        k = 10
        z = 0
        for j in range(3):
            print(j)
            body = {
                "range": {
                    "from": 0,
                    "to": 3,
                    "step": 1,
                },
                "parameters": [3, 2],
                "sequences": []
                #     {
                #         "name": "Arithmetic",
                #         "parameters": [z, k],
                #         "sequences": [
                #             {
                #                 "name": "Arithmetic",
                #                 "parameters": [z, k],
                #                 "sequences": [],
                #             }
                #         ],
                #     },
                # ],
            }
            print(j)
            r = requests.post(url + "/Arithmetic", json=body)
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
