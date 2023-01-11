from math import sqrt, floor
limit = 2000000


def isPrime(num):
    if(num == 1):
        return False
    for i in range(2, floor(sqrt(num)) + 1):
        if (num % i == 0):
            return False
    return True


total = 0
for i in range(limit):
    if(isPrime(i)):
        total += i
    

print(total)
