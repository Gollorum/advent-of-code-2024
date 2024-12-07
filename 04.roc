app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/04.txt" as input : Str
import Utils

grid = Str.trim input |> Str.splitOn "\n" |> List.map \line -> Str.toUtf8 line

eachCoordinateWhere: (U8, U64, U64 -> Bool) -> List (U64, U64)
eachCoordinateWhere = \predicate ->
    Utils.joinMapWithIndex grid \line, y ->
        coordsWithBool = List.mapWithIndex line \a, x -> (predicate a x y, x)
        filteredWithBool = List.keepIf coordsWithBool \(b, _x) -> b
        List.map filteredWithBool \(_b, x) -> (x, y)

tryCollectInDirection = \(x0, y0), (xDir, yDir) ->
    x1 = Num.toI64 x0 + xDir
    y1 = Num.toI64 y0 + yDir
    first = List.get grid (Num.toU64 y1) |> Result.try? \line -> List.get line (Num.toU64 x1)
    x2 = x1 + xDir
    y2 = y1 + yDir
    second = List.get grid (Num.toU64 y2) |> Result.try? \line -> List.get line (Num.toU64 x2)
    x3 = x2 + xDir
    y3 = y2 + yDir
    third = List.get grid (Num.toU64 y3) |> Result.try? \line -> List.get line (Num.toU64 x3)
    Ok (first, second, third)

tryCollectSideways = \(x0, y0), (xDir, yDir) ->
    x1 = Num.toI64 x0 + xDir
    y1 = Num.toI64 y0 + yDir
    first = List.get grid (Num.toU64 y1) |> Result.try? \line -> List.get line (Num.toU64 x1)
    x2 = Num.toI64 x0 - xDir
    y2 = Num.toI64 y0 - yDir
    second = List.get grid (Num.toU64 y2) |> Result.try? \line -> List.get line (Num.toU64 x2)
    Ok (first, second)

part1 =
    directionsToCheck: List (I64, I64)
    directionsToCheck = [ (1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1) ]
    candidates = eachCoordinateWhere \char, _x, _y -> char == 'X'
    words = List.joinMap candidates \c -> List.keepOks directionsToCheck \dir -> tryCollectInDirection c dir
    numwords = List.countIf words \word -> word == ('M', 'A', 'S')
    Stdout.line! "count: $(Num.toStr numwords)"

part2 =
    directionsToCheck: List (I64, I64)
    directionsToCheck = [ (1, -1), (1, 1), (-1, -1), (-1, 1) ]
    candidates = eachCoordinateWhere \char, _x, _y -> char == 'A'
    wordsPerCoord = List.map candidates \c -> List.keepOks directionsToCheck \dir -> tryCollectSideways c dir
    numwords = List.countIf wordsPerCoord \words -> 2 <= List.countIf words \word -> word == ('M', 'S')
    Stdout.line! "count: $(Num.toStr numwords)"

main = part2