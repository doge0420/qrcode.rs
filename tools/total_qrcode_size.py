import requests
from bs4 import BeautifulSoup

page = requests.get(
    "https://www.thonky.com/qr-code-tutorial/error-correction-table")

soup = BeautifulSoup(page.content, 'html.parser')

table = soup.find('table', class_='table table-bordered')

lines = table.find_all('tr')[1:]

qr_code_size = []

for line in lines:
    cells = line.find_all('td')
    qr_code_size.append(cells[1].text)

ec_l = []
ec_m = []
ec_q = []
ec_h = []

for i, size in enumerate(qr_code_size):
    if i % 4 == 0:
        ec_l.append(size)
    elif i % 4 == 1:
        ec_m.append(size)
    elif i % 4 == 2:
        ec_q.append(size)
    elif i % 4 == 3:
        ec_h.append(size)

print(
    f"static SIZE_EC_L: [u32; {len(ec_l)}] = {list(map(lambda x: int(x), ec_l))};\n"
)

print(
    f"static SIZE_EC_M: [u32; {len(ec_m)}] = {list(map(lambda x: int(x), ec_m))};\n"
)

print(
    f"static SIZE_EC_Q: [u32; {len(ec_q)}] = {list(map(lambda x: int(x), ec_q))};\n"
)

print(
    f"static SIZE_EC_H: [u32; {len(ec_h)}] = {list(map(lambda x: int(x), ec_h))};\n"
)
