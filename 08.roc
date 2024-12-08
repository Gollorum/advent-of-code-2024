app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/08.txt" as input : Str
import Utils

Pos : {x: U64, y: U64}

result =
    grid =
        Str.trim input
        |> Str.splitOn "\n"
        |> List.map \line -> Str.toUtf8 line
    antennas = collectAntennas grid
    height = List.len grid
    width = List.get grid 0 |> Result.map? List.len
    antinodes = collectAntinodesPart2 antennas width height
    Ok (Set.len antinodes |> Num.toStr)

collectAntinodesPart1 = \antennas, width, height ->
    Dict.walk antennas (Set.empty {}) \set, _key, positions ->
        List.walk positions set \set2, pos ->
            List.walk positions set2 \set3, pos2 ->
                if pos != pos2 then
                    xRes = Num.subChecked (2 * pos.x) pos2.x
                    yRes = Num.subChecked (2 * pos.y) pos2.y
                    when (xRes, yRes) is
                        (Ok x, Ok y) if x < width && y < height -> Set.insert set3 {x: x, y: y}
                        _ -> set3
                else
                    set3

collectAntinodesPart2 = \antennas, width, height ->
    List.range { start: At 0, end: At (width - 1) }
    |> List.joinMap \x ->
        List.range { start: At 0, end: At (height - 1) }
        |> List.keepIf \y ->
            Utils.dictAny antennas \positions ->
                Utils.allPairs positions |> List.any \(pos1, pos2) ->
                    areInLine pos1 pos2 {x: x, y: y}
        |> List.map \y -> {x: x, y: y}
    |> Set.fromList

areInLine : Pos, Pos, Pos -> Bool
areInLine = \pos1, pos2, pos3 ->
    dx = Num.toI64 pos2.x - Num.toI64 pos1.x
    dy = Num.toI64 pos2.y - Num.toI64 pos1.y
    dx2 = Num.toI64 pos3.x - Num.toI64 pos1.x
    dy2 = Num.toI64 pos3.y - Num.toI64 pos1.y
    dx * dy2 == dx2 * dy

collectAntennas: List (List U8) -> Dict U8 (List Pos)
collectAntennas = \grid ->
    List.walkWithIndex grid (Dict.empty {}) \dict, line, y ->
        List.walkWithIndex line dict \dict2, char, x ->
            when char is
                '.' -> dict2
                _ -> Utils.appendIn dict2 char {x: x, y: y}

main = when result is
    Ok r -> Stdout.line! r
    Err _ -> Stdout.line! "Unknown error"
