import pandas as pd

def clear():
    open("averages.js", "w").close()

def append(contents):
    file = open("averages.js", "a")
    file.write(contents)
    file.close()

def process(elections):
    data = pd.read_csv(f"{elections}_ELECTIONS_2025.csv")
    data = data[data["Party"] != ""]
    subsets = {party: subset for party, subset in data.groupby("Party")}
    averages = {party: subset.iloc[:, 7:32].mean(skipna = True).tolist() for party, subset in subsets.items()}
    append(f"const {elections} = {averages}\n")

clear()
process("COUNTY")
process("MUNICIPAL")
