numbers = list(map(int, open("numbers.txt").read().splitlines()))
print(str(sum(numbers))[:10])