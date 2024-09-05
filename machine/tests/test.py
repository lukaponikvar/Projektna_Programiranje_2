# TODO: Naredi, da poženeš zgolj to datoteko, in ona požene vse test file in jih primerja med sabo.


import requests


url = "http://127.0.0.1:12345/sequence/"

#######################################################################################################################
# Constant
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 10,
        "step": 1,
    },
    "parameters": [1.0],
    "sequences": []
}
r = requests.post(url + "Constant", json=body)
assert r.text == "[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0]"

#######################################################################################################################
# Arithmetic
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 10,
        "step": 1,
    },
    "parameters": [0.0, 1.0],
    "sequences": []
}
r = requests.post(url + "Arithmetic", json=body)
assert r.text == "[0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0]"

body = {
    "range": {
        "from": 0,
        "to": 3,
        "step": 1,
    },
    "parameters": [3, 2],
    "sequences": []
}
r = requests.post(url + "Arithmetic", json=body)
assert r.text == "[3.0,5.0,7.0,9.0]"

body = {
    "range": {
        "from": 7,
        "to": 24,
        "step": 8,
    },
    "parameters": [3, 264],
    "sequences": []
}
r = requests.post(url + "Arithmetic", json=body)
assert r.text == "[1851.0,3963.0,6075.0]"

body = {
    "range": {
        "from": 0,
        "to": 13,
        "step": 1,
    },
    "parameters": [5.0, -5.0],
    "sequences": []
}
r = requests.post(url + "Arithmetic", json=body)
assert r.text == "[5.0,0.0,-5.0,-10.0,-15.0,-20.0,-25.0,-30.0,-35.0,-40.0,-45.0,-50.0,-55.0,-60.0]"

#######################################################################################################################
# Geometric
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 3,
        "step": 1,
    },
    "parameters": [2.0, 2.0],
    "sequences": []
}
r = requests.post(url + "Geometric", json=body)
assert r.text == "[2.0,4.0,8.0,16.0]"

#######################################################################################################################
# Sum
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 3,
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
            "name": "Constant",
                    "parameters": [4],
                    "sequences": [],
        },
    ],
}
r = requests.post(url + "Sum", json=body)
assert r.text == "[7.0,9.0,11.0,13.0]"

#######################################################################################################################
# Product
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 3,
        "step": 1,
    },
    "parameters": [],
    "sequences": [
        {
            "name": "Arithmetic",
                    "parameters": [1, 2],
                    "sequences": [],
        },
        {
            "name": "Constant",
                    "parameters": [2],
                    "sequences": [],
        },
    ],
}
r = requests.post(url + "Product", json=body)
assert r.text == "[2.0,6.0,10.0,14.0]"
# print(r.text)
