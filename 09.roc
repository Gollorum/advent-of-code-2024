app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/09.txt" as input : Str

parseDigit: U8 -> Result U8 [InvalidDigit]
parseDigit = \c ->
    if c >= '0' && c <= '9' then
        Ok (c - '0')
    else
        Err InvalidDigit

collectNewDiscMap: List (U64, U8), List U8, (U64, U8), (U64, U8) -> Result (List (U64, U8)) [OutOfBounds]
collectNewDiscMap = \accum, diskMap, (spaceI, spaceLeft), (rightI, dataLeft) ->
    if spaceLeft > dataLeft then
        newAccum = List.append accum (rightI, dataLeft)
        newRightI = rightI - 2
        if newRightI < spaceI then
            Ok newAccum
        else
            #newDataLeft = List.get? diskMap newRightI
            #collectNewDiscMap newAccum diskMap (spaceI, spaceLeft - dataLeft) (newRightI, newDataLeft)
            advanceRight newAccum diskMap spaceI spaceLeft dataLeft newRightI
    else
        newSpaceI = spaceI + 2
        if newSpaceI > rightI then
            List.append accum (rightI, dataLeft)
            |> Ok
        else
            leftToAppend = List.get? diskMap (spaceI + 1)
            newSpaceLeft = List.get? diskMap newSpaceI
            newAccum = List.append accum (rightI, spaceLeft)
                |> List.append (spaceI + 1, leftToAppend)
            collectNewDiscMap newAccum diskMap (newSpaceI, newSpaceLeft) (rightI, dataLeft - spaceLeft)

advanceRight: List (U64, U8), List U8, U64, U8, U8, U64 -> Result (List (U64, U8)) [OutOfBounds]
advanceRight = \accum, diskMap, spaceI, spaceLeft, dataLeft, newRightI ->
    List.get diskMap newRightI
    |> Result.try? \newDataLeft ->
        collectNewDiscMap accum diskMap (spaceI, spaceLeft - dataLeft) (newRightI, newDataLeft)

gaussSum: Int a -> Int a
gaussSum = \n -> n * (n + 1) |> Num.divTrunc 2

result =
    diskMap = Str.trim input |> Str.toUtf8 |> List.mapTry? parseDigit
    firstSpace = List.get? diskMap 1
    firstData = List.get? diskMap 0
    lastData = List.get? diskMap (List.len diskMap - 1)
    newDiscMap =
        collectNewDiscMap [] diskMap (1, firstSpace) (List.len diskMap - 1, lastData)
        |> Result.map? \newMap ->
            List.keepIf newMap \(_, amount) -> amount != 0
            |> List.map \(i, amount) -> (Num.divTrunc i 2, amount)
    (res, _) = List.walk newDiscMap (0, Num.toU128 firstData) \(sum, offset), (fileId, amount) ->
        newOffset = offset + Num.toU128 amount
        summand = (gaussSum (newOffset - 1) - gaussSum (offset-1))
        (sum + summand * Num.toU128 fileId, newOffset)
        #(gaussSum (index + Num.toU64 amount) - gaussSum (index)) * fileId
    Num.toStr res
    |> Ok

main =
    when result is
        Ok res -> Stdout.line! res
        Err _ -> Stdout.line! "Unknown error"
