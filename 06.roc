app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/06.txt" as input : Str

Pos : {x: I64, y: I64}
Dir : {x: I8, y: I8}

report =
    lines = Str.splitOn input "\n"
    map = List.mapTry? lines parseLine
    start = findStartPos? lines
    cyclics = List.countIf (variants map start) \variant -> isCyclic variant (Set.empty {}) (start, {x:0, y:-1})
#    totalBlocked = stepUntilDone map (Set.empty {} |> Set.insert start, start, {x:0, y:-1})
#    "blocked:$(Num.toStr (Set.len totalBlocked))"
    "Cyclics? $(Num.toStr cyclics)"
    |> Ok

isCyclic: List (List Bool), Set (Pos, Dir), (Pos, Dir) -> Bool
isCyclic = \map, prevs, state ->
    if Set.contains prevs state then
        Bool.true
    else
        when step map state is
            Err Done -> Bool.false
            Ok nextState -> isCyclic map (Set.insert prevs state) nextState

step: List (List Bool), (Pos, Dir) -> Result (Pos, Dir) [Done]
step = \map, (pos, dir) ->
    nextPos = add pos dir
    when inspectPos map nextPos is
        Outside -> Err Done
        Free -> Ok (nextPos, dir)
        Blocked -> Ok (pos, {x: -dir.y, y: dir.x})

variants: List (List Bool), Pos -> List (List (List Bool))
variants = \map, start ->
    joinMapWithIndex map \row, y ->
        List.mapWithIndex row \isBlocked, x ->
            if isBlocked || (y == Num.toU64 start.y && x == Num.toU64 start.x) then
                Err Nope
            else
                List.set map y (List.set row x Bool.true)
                |> Ok
        |> List.keepOks \it -> it

stepUntilDone: List (List Bool), (Set Pos, Pos, Dir) -> Set Pos
stepUntilDone = \map, (visited, pos, dir) -> when stepAndRemember map (visited, pos, dir) is
    Ok next -> stepUntilDone map next
    Err Done -> visited

stepAndRemember: List (List Bool), (Set Pos, Pos, Dir) -> Result (Set Pos, Pos, Dir) [Done]
stepAndRemember = \map, (visited, pos, dir) ->
    nextPos = add pos dir
    when inspectPos map nextPos is
        Outside -> Err Done
        Free -> Ok (Set.insert visited nextPos, nextPos, dir)
        Blocked -> Ok (visited, pos, {x: -dir.y, y: dir.x})

main = when report is
    Ok r -> Stdout.line! r
    Err _ -> Stdout.line! "Unknown error"

parseLine = \line -> Str.toUtf8 line |> List.mapTry isTileBlocked

isTileBlocked = \c -> when c is
    '.' -> Ok Bool.false
    '^' -> Ok Bool.false
    '#' -> Ok Bool.true
    _ -> Err (UnknownTile c)

findStartPos: List Str -> Result Pos [NotFound, OutOfBounds]
findStartPos = \lines ->
    startY = List.findFirstIndex? lines \line -> Str.contains line "^"
    line = List.get? lines startY
    startX = Str.toUtf8 line |> List.findFirstIndex? \c -> c == '^'
    x = Num.toI64Checked? startX
    y = Num.toI64Checked? startY
    Ok { x: x, y: y }

add: Pos, Dir -> Pos
add = \a, b -> {x: a.x + Num.toI64 b.x, y: a.y + Num.toI64 b.y}

inspectPos: List (List Bool), Pos -> [Free, Blocked, Outside]
inspectPos = \map, pos ->
    state =
        x = Num.toU64Checked? pos.x
        y = Num.toU64Checked? pos.y
        line = List.get? map y
        List.get line x
    when state is
        Err OutOfBounds -> Outside
        Ok isBlocked -> if isBlocked then Blocked else Free

joinMapWithIndex: List a, (a, U64 -> List b) -> List b
joinMapWithIndex = \list, mapping ->
    List.mapWithIndex list mapping
    |> List.joinMap \it -> it