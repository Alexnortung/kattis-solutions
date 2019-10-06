last = input()
firstLetter = last[-1]
amount = int(input())
animals = [False for i in range(26)]
a = ord('a')
checkAnimals = []

for i in range(amount):
    animal = input()
    cFirstLetter = animal[0]
    animals[ord(cFirstLetter) - a] = True

    if cFirstLetter == firstLetter:
        checkAnimals.append(animal)

found = False
checkAnimalLength = len(checkAnimals)

if checkAnimalLength == 0:
    print("?")
    found = True

if checkAnimalLength == 1:
    animal = checkAnimals[0]
    if firstLetter == animal[-1]:
        print(animal + "!")
        found = True

if not found:
    for animal in checkAnimals:
        newLetter = animal[-1]
        if animals[ord(newLetter) - a] == False:
            print(animal + "!")
            found = True
            break

if not found:
    print(checkAnimals[0])
