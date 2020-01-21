import math

splittedInput = input().split()
splittedInput = [int(i) for i in splittedInput]
m, n, a = splittedInput
found = False
if a % m == 0 or a % n == 0:
    print(1)
    found = True

#check
if not found:

    numStop = math.ceil(math.sqrt(a))
    numCheck = 1
    minSide = min(m, n)
    maxSide = max(m, n)

    while numCheck <= numStop and numCheck <= minSide:
        numCheck2 = a / numCheck
        #if numCheck2 % 1 != 0:
        #    numCheck += 1
        #    continue

        if numCheck2 % 1 == 0 and numCheck2 <= maxSide:
            print(2)
            found = True
            break

        
        newMin = minSide - numCheck
        if newMin >= 1:
            firstBreak = numCheck * maxSide
            newA = a - firstBreak
            maxSideCheck = newA / newMin
            #check if a divides minSide
            #a cant divide maxSide at this point
            #print(numCheck)
            
                        
            if maxSideCheck % 1 == 0 and newA % newMin == 0 and maxSideCheck <= maxSide and newA > 0:
                #print(maxSideCheck)
                print(2)
                found = True
                break


        numCheck += 1

if not found:
    print(3)
