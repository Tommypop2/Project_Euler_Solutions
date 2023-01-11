from itertools import permutations
digits = [0,1,2,3,4,5,6,7,8,9]
nums = []
def convertLstToNum(lst: list[int]):
    strNum = ""
    for n in lst:
        strNum += str(n)
    return int(strNum)
print(convertLstToNum(list(permutations(digits))[999999]))