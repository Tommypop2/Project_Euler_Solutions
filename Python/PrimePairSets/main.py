from math import sqrt, floor


def isPrime(num):
    num = int(num)
    isPrime = True
    for n in range(2, floor(sqrt(num)) + 1):
        if ((num / n) % 1 == 0):
            isPrime = False
    return isPrime


def getPrimes():
    i = 1
    while True:
        i += 1
        if (isPrime(i) == True):
            yield i


def doConcat(arr):
    for i in range(len(arr) - 1):
        index = i + 1
        while index > 0:
            if(not isPrime(str(arr[i]) + str(arr[index])) and not isPrime(str(arr[i]) + str(arr[index]))):
                return False
            index -= 1
    return True


primes = []
for i in getPrimes():
    if (i > 10000):
        break
    primes.append(i)

for i in primes:
    print(i)
    for n in primes:
        print(n)
        if(doConcat([i,n])):
            for x in primes:
                if(doConcat([i,n,x])):
                    for y in primes:
                        if(doConcat([i,n,x,y])):
                            print(i,n,x,y)
