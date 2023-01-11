# Project Euler Problem 44
def pe44():
    ps = set() #Set of Pentagonal Numbers
    i = 0
    while True:
        i += 1
        s = (3*i*i - i) // 2 #Generate pentagonal number which will be used as the sum
        for Pj in ps:
            if s-Pj in ps and s-2*Pj in ps: #Checks if the sum - Pj is present, which would be Pk, and checks if the difference is present, then returns the difference.
                return s-2*Pj
        ps.add(s)


print("Project Euler 44 Solution =", pe44())
