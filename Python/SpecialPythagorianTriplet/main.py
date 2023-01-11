from math import sqrt
for a in range(1000):
    for b in range(1000-a):
        if(a + b + sqrt(a**2 + b**2) == 1000 and sqrt(a**2 + b**2) % 1 == 0 and a < b and a > 0 and b > 0):
            c = sqrt(a**2 + b**2)
            print(a * b * c)