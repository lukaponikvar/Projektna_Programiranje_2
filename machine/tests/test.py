import requests

url = "http://127.0.0.1:9000/sequence/"

#######################################################################################################################
# Constant
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 11,
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
        "to": 11,
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
        "to": 4,
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
        "to": 25,
        "step": 8,
    },
    "parameters": [3, 264],
    "sequences": [{
        "name": "Geometric",
        "parameters": [1, 2],
        "sequences": [],
    },]
}
r = requests.post(url + "Arithmetic", json=body)
assert r.text == "Requested an invalid sequence.\nSequence name: Arithmetic does match one of ours, nevertheless number of given sequences: 1 is not equal to the required number of sequences: 0.\n"

body = {
    "range": {
        "from": 0,
        "to": 14,
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
        "to": 4,
        "step": 1,
    },
    "parameters": [2.0, 2.0],
    "sequences": []
}
r = requests.post(url + "Geometric", json=body)
assert r.text == "Requested an invalid sequence.\nSequence name: Geometric does not match any of ours.\n"

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
            "name": "Geometric",
                    "parameters": [1, 2],
                    "sequences": [],
        },
    ],
}
r = requests.post(url + "Sum", json=body)
# print(r.text)
assert r.text == "[4.0,7.0,11.0]"

#######################################################################################################################
# Product
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 4,
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

#######################################################################################################################
# Drop
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 4,
        "step": 1,
    },
    "parameters": [2],
    "sequences": [
        {
            "name": "Arithmetic",
                    "parameters": [1, 2],
                    "sequences": [],
        }
    ],
}
r = requests.post(url + "Drop", json=body)
assert r.text == "[5.0,7.0,9.0,11.0]"
# print(r.text)

#######################################################################################################################
# Average
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 7,
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
r = requests.post(url + "Average", json=body)
assert r.text == "[1.5,2.5,3.5,4.5,5.5,6.5,7.5]"
# print(r.text)

#######################################################################################################################
# Fibonacci
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 10,
        "step": 1,
    },
    "parameters": [0, 1],
    "sequences": [],
}
r = requests.post(url + "Fibonacci", json=body)
assert r.text == "[0.0,1.0,1.0,2.0,3.0,5.0,8.0,13.0,21.0,34.0]"
# print(r.text)

#######################################################################################################################
# Linear Combination
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 7,
        "step": 1,
    },
    "parameters": [2, 5],
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
r = requests.post(url + "LinearCombination", json=body)
assert r.text == "[12.0,16.0,20.0,24.0,28.0,32.0,36.0]"
# print(r.text)

#######################################################################################################################
# Min
#######################################################################################################################


body = {
    "range": {
        "from": 0,
        "to": 8,
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
                    "parameters": [4],
                    "sequences": [],
        },
    ],
}
r = requests.post(url + "Min", json=body)
assert r.text == "[1.0,3.0,4.0,4.0,4.0,4.0,4.0,4.0]"
# print(r.text)


max


body = {
    "range": {
        "from": 0,
        "to": 5,
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
                    "parameters": [4],
                    "sequences": [],
        },
    ],
}
r = requests.post(url + "Max", json=body)
assert r.text == "[4.0,4.0,5.0,7.0,9.0]"
# print(r.text)
