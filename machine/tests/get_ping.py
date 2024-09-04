import requests

sequences = requests.get("http://127.0.0.1:12345/ping")
print(sequences.text)
