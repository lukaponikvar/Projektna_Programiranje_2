import requests

url = "http://127.0.0.1:9000/sequence/"

#######################################################################################################################
# Constant
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 11,
        "step": 2,
    },
    "parameters": [1.0],
    "sequences": []
}
r = requests.post(url + "Constant/", json=body)
# print(r.text)
assert r.text == "[1.0,1.0,1.0,1.0,1.0,1.0]"

#######################################################################################################################
# Arithmetic
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 12,
        "step": 2,
    },
    "parameters": [0.0, 1.0],
    "sequences": []
}
r = requests.post(url + "Arithmetic", json=body)

assert r.text == "[0.0,2.0,4.0,6.0,8.0,10.0]"

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
# print(r.text)
# assert r.text == "Invalid input format.\nSequence name: Arithmetic does match one of ours, nevertheless number of given sequences: 1 is not equal to the required number of sequences: 0.\n"

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
        "to": 5,
        "step": 2,
    },
    "parameters": [2.0, 2.0],
    "sequences": []
}
r = requests.post(url + "Geometric", json=body)
assert r.text == "[2.0,8.0,32.0]"

#######################################################################################################################
# Sum
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 7,
        "step": 2,
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
assert r.text == "[4.0,11.0,27.0,79.0]"

#######################################################################################################################
# Product
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 8,
        "step": 2,
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
assert r.text == "[2.0,10.0,18.0,26.0]"

#######################################################################################################################
# Drop
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 4,
        "step": 2,
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
assert r.text == "[5.0,9.0]"

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

#######################################################################################################################
# Linear Combination
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 7,
        "step": 1,
    },
    "parameters": [2, 5, 1],
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
assert r.text == "[13.0,17.0,21.0,25.0,29.0,33.0,37.0]"

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

#######################################################################################################################
# Max
#######################################################################################################################

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

#######################################################################################################################
# Random
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 5,
        "step": 1,
    },
    "parameters": [31.1, 1.32222222222222222222234134143413412412422984129874681734615668270918284687126438917241824681925],
    "sequences": [],
}
r = requests.post(url + "Random", json=body)
print(r.text)

#######################################################################################################################
# Floor
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 7,
        "step": 1,
    },
    "parameters": [],
    "sequences": [{
        "name": "Average",
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
    }],
}
r = requests.post(url + "Floor", json=body)
assert r.text == "[1.0,2.0,3.0,4.0,5.0,6.0,7.0]"

#######################################################################################################################
# Neki
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 7,
        "step": 1,
    },
    "parameters": [0],
    "sequences": [{
        "name": "Floor",
        "parameters": [],
        "sequences": [{
            "name": "Average",
            "parameters": [],
            "sequences": [
                {
                    "name": "Arithmetic",
                    "parameters": [1.0, 2],
                    "sequences": [],
                },
                {
                    "name": "Constant",
                    "parameters": [2],
                    "sequences": [],
                },
            ],
        }],
    }]}
r = requests.post(url + "Drop", json=body)
assert r.text == "[1.0,2.0,3.0,4.0,5.0,6.0,7.0]"


#######################################################################################################################
# Binary
#######################################################################################################################

body = {
    "range": {
        "from": 0,
        "to": 7,
        "step": 1,
    },
    "parameters": [],
    "sequences": [{
        "name": "Floor",
        "parameters": [],
        "sequences": [{
            "name": "Average",
            "parameters": [],
            "sequences": [
                {
                    "name": "Arithmetic",
                    "parameters": [1.0, 42],
                    "sequences": [],
                },
                {
                    "name": "Constant",
                    "parameters": [2],
                    "sequences": [],
                },
            ],
        }],
    }]}
r = requests.post(url + "Binary", json=body)
# print(r.text)
assert r.text == "[1.0,10110.0,101011.0,1000000.0,1010101.0,1101010.0,1111111.0]"
