from math import sqrt


def generateNumber(n):
    return (n * (3 * n - 1)) // 2


def isNumPentagonal(x):
    val1 = (1 + (sqrt(1 + 24 * x))) / 6
    if (val1.is_integer()):
        return val1
    else:
        return False

for k in range(1, 5000):
    for j in range(1, 5000):
        num1 = generateNumber(k)
        num2 = generateNumber(j)
        sum = num1 + num2
        dif = abs(num1 - num2)
        if (isNumPentagonal(sum) and isNumPentagonal(dif) and k != j):
            print(dif)
            exit(0)