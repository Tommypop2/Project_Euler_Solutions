grid = [list(map(int, x.split(" ")))
        for x in open("data.txt").read().splitlines()]
greatestProduct = 0
currentProd = 1
for i in range(20):
    for n in range(20):
        currentProd = 1
        if (n < 17):
            for x in range(4):  # Right
                currentProd *= grid[i][n+x]
            if (currentProd > greatestProduct):
                greatestProduct = currentProd
        currentProd = 1
        if (i < 17):
            for x in range(4):  # Down
                currentProd *= grid[i+x][n]
            if (currentProd > greatestProduct):
                greatestProduct = currentProd
        currentProd = 1
        if (n < 17 and i < 17):
            for x in range(4):  # Diagonal Right
                currentProd *= grid[i+x][n+x]
            if (currentProd > greatestProduct):
                greatestProduct = currentProd
        currentProd = 1
        if (n > 3 and i < 17):
            for x in range(4):  # Diagonal Left
                currentProd *= grid[i+x][n-x]
            if (currentProd > greatestProduct):
                greatestProduct = currentProd
print(greatestProduct)
