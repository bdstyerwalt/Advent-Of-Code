InputList = []
with open("Input.txt", "r") as data:
    for t in data:
        Line = t.strip()
        InputList.append(Line)

SymbolSet = set()
SymbolLocations = set()
GearLocations = set()
for y, i in enumerate(InputList):
    for x, k in enumerate(i):
        if k != "." and not(k.isnumeric()):
            SymbolSet.add(k)
            SymbolLocations.add((x, y))
            if k == "*":
                GearLocations.add((x, y))

NumberList = []
NumberActive = False
Width = len(InputList[0])
for y, l in enumerate(InputList):
    for x, c in enumerate(l):
        if not(NumberActive) and (c == "." or c in SymbolSet):
            continue
        elif not(NumberActive) and c.isnumeric():
            CurrentNumber = c
            NumberActive = True
            NewX1 = x
        elif NumberActive and (c == "." or c in SymbolSet):
            NewNum = int(CurrentNumber)
            NewX2 = x-1
            NumberList.append((NewNum, NewX1, NewX2, y))
            NumberActive = False
            CurrentNumber = ""
        elif NumberActive and c.isnumeric():
            CurrentNumber += c
            if x == Width - 1:
                NewNum = int(CurrentNumber)
                NewX2 = x
                NumberList.append((NewNum, NewX1, NewX2, y))
                NumberActive = False
                CurrentNumber = ""

Part1Answer = 0
for Number, X1, X2, Y in NumberList:
    BorderPoints = set()
    for y in range(Y-1,Y+2):
        for x in range(X1-1,X2+2):
            BorderPoints.add((x, y))
    IntersectSet = SymbolLocations & BorderPoints
    if len(IntersectSet) > 0:
        Part1Answer += Number

Part2Answer = 0
for GX, GY in GearLocations:
    GearBorders = set()
    GearNeighbors = []
    for x in range(GX-1,GX+2):
        for y in range(GY-1,GY+2):
            GearBorders.add((x,y))
    for Number, X1, X2, Y in NumberList:
        if (X1, Y) in GearBorders or (X2, Y) in GearBorders:
            GearNeighbors.append(Number)
    if len(GearNeighbors) == 2:
        N1, N2 = GearNeighbors
        Part2Answer += N1*N2

print(Part1Answer)
print(Part2Answer)