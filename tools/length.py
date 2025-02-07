import requests
from bs4 import BeautifulSoup

page = requests.get("https://www.thonky.com/qr-code-tutorial/character-capacities")

soup = BeautifulSoup(page.content, 'html.parser')

table = soup.find('table', class_='table table-bordered')

lines = table.find_all('tr')[1:]

numeric = []
alphanumeric = []
byte = []
kanji = []

for i, line in enumerate(lines):
    cells = line.find_all('td')
    if i % 4 == 0:
        cells = cells[1:]

    numeric.append(cells[1].text)
    alphanumeric.append(cells[2].text)
    byte.append(cells[3].text)
    kanji.append(cells[4].text)
    
print(
    f"const NUMERIC_SIZE: [u32; {len(numeric)}] = {list(
        map(lambda x: int(x), numeric))};\n"
)

print(
    f"const ALPHANUMERIC_SIZE: [u32; {len(alphanumeric)}] = {list(
        map(lambda x: int(x), alphanumeric))};\n"
)

print(
    f"const BYTE_SIZE: [u32; {len(byte)}] = {list(
        map(lambda x: int(x), byte))};\n"
)

print(
    f"const KANJI_SIZE: [u32; {len(kanji)}] = {list(
        map(lambda x: int(x), kanji))};\n"
)
