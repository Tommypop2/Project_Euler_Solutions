from math import sqrt
maxPrimes = 0


def isPrime(num):
    for i in range(2, int(sqrt(abs(num)) + 1)):
        if (num % i == 0):
            return False
    return True


def getNumOfPrimes(a, b):
    n = 0
    while True:
        res = n ** 2 + n * a + b
        if (not isPrime(res)):
            return n
        n += 1


bestExpression = [0, 0, 0]
for a in range(-999, 1000):
    for b in range(-1000, 1001):
        primes = getNumOfPrimes(a, b)
        if (bestExpression[0] < primes):
            bestExpression = [primes, a, b]
prod = 1
for i in bestExpression[1:]:
    prod *= i
print(prod)
print(bestExpression[0])
operators = ["+", "-"]
print(print(f"n² {operators[0] if bestExpression[1] > 0 else operators[1]} {abs(bestExpression[1])}n {operators[0] if bestExpression[2] > 0 else operators[1]} {abs(bestExpression[2])}"))
