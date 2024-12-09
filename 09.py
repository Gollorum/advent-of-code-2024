with open('inputs/09.txt') as f:
    line = f.readline()

diskMap = [int(x) for x in line]

def gaussSum(n):
    return n * (n + 1) // 2

def part1():
    newDiscMap = [(0, diskMap[0])]
    spaceI = 1
    spaceLeft = diskMap[1]
    dataI = len(diskMap) - 1
    dataLeft = diskMap[dataI]
    def appendFile(index, count):
        if count > 0:
            newDiscMap.append((index // 2, count))

    while spaceI < dataI:
        if spaceLeft > dataLeft or spaceI + 1 == dataI:
            appendFile(dataI, dataLeft)
            dataI -= 2
            spaceLeft -= dataLeft
            dataLeft = diskMap[dataI]
        else:
            appendFile(dataI, spaceLeft)
            dataLeft -= spaceLeft
            appendFile(spaceI + 1, diskMap[spaceI + 1])
            spaceI += 2
            spaceLeft = diskMap[spaceI]

    sum = 0
    offset = 0
    for (fileId, count) in newDiscMap:
        newOffset = offset + count
        sum += (gaussSum(newOffset - 1) - gaussSum(offset - 1)) * fileId
        offset = newOffset

    return sum

def part2(): # Works for the sample case, but not for the actual input
    newDiscMap = []
    for i in range(len(diskMap)):
        if i % 2 == 0:
            newDiscMap += [i // 2] * diskMap[i]
        else:
            newDiscMap += [None] * diskMap[i]
    r = len(newDiscMap) - 1
    i = r
    while r > 0:
        while r > 0 and (newDiscMap[r] is None or newDiscMap[r] > i):
            r -= 1
        rr = r
        i = newDiscMap[r]
        while r > 0 and newDiscMap[r] == i:
            r -= 1
        if r <= 0:
            break
        count = rr - r
        lcount = 0
        l = 0
        while lcount < count:
            while l < len(newDiscMap) and newDiscMap[l] is not None:
                l += 1
            ll = l
            while l < len(newDiscMap) and newDiscMap[l] is None:
                l += 1
            if l > r:
                break
            lcount = l - ll
        if lcount < count:
            continue
        for o in range(count):
            newDiscMap[ll + o] = i
            newDiscMap[rr - o] = None
    sum = 0
    for i in range(len(newDiscMap)):
        if newDiscMap[i] is not None:
            sum += i * newDiscMap[i]
    return sum

def part2v2(): # works
    newDiscMap = [(None if i % 2 == 1 else i // 2, diskMap[i]) for i in range(len(diskMap))]
    i = len(newDiscMap) - 1
    id = i
    while i > 0:
        if newDiscMap[i][0] is None or newDiscMap[i][0] > id:
            i -= 1
            continue
        (id, count) = newDiscMap[i]
        newIndex = None
        for j in range(1, i):
            if newDiscMap[j][0] is None and newDiscMap[j][1] >= count:
                newIndex = j
                break
        if newIndex is not None:
            newDiscMap[i] = (None, count)
            if newDiscMap[newIndex][1] == count:
                newDiscMap[newIndex] = (id, count)
                i -= 1
            else:
                newDiscMap[newIndex] = (None, newDiscMap[newIndex][1] - count)
                newDiscMap.insert(newIndex, (id, count))
        else:
            i -= 1
    sum = 0
    offset = 0
    for (fileId, count) in newDiscMap:
        newOffset = offset + count
        if fileId is not None:
            sum += (gaussSum(newOffset - 1) - gaussSum(offset - 1)) * fileId
        offset = newOffset

    return sum


print(part1())
print(part2v2())