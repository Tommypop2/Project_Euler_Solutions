def countSolutions(n):
    x = 10 ** n
    b = 1
    count = 0
    while True:
        for a in range(1, b + 1):
            if((x * (b + a) % (a * b)) == 0):
                p = x * (b + a) / (a * b)
                count += 1
                print(count)
        b += 1

countSolutions(9)