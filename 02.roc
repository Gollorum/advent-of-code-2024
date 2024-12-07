app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/02.txt" as input : Str
import Utils

isSafeDiff: Int Signed32, Int Signed32 -> [Inc, Dec, Unsafe]
isSafeDiff = \a, b ->
    diff = b - a
    if  diff < -3 || diff == 0 || diff > 3 then
        Unsafe
    else if diff < 0 then
        Dec
    else
        Inc

combineStates: [Inc, Dec, Unsafe, Init, Start], [Inc, Dec, Unsafe] -> [Inc, Dec, Unsafe, Init, Start]
combineStates = \a, b ->
    when (a, b) is
        (Inc, Inc) -> a
        (Dec, Dec) -> a
        _ -> Unsafe

tryParse: Str -> Result (Int Signed32) Str
tryParse = \str -> when Str.toI32 str is
    Ok res -> Ok res
    Err _ -> Err "Failed to parse |$(str)|"

versionsOf: List (Int Signed32) -> List (List (Int Signed32))
versionsOf = \levels -> List.mapWithIndex levels \_, i -> List.dropAt levels i

result =
    reports = Str.trim input |> Str.splitOn "\n" |> List.mapTry? \line -> Str.splitOn line " " |> List.mapTry tryParse
    safeCount = List.countIf reports \rawLevels -> Utils.anyTrue (List.map (versionsOf rawLevels) \levels ->
        (resultState, _) = List.walk levels (Init, 0) \(state, lastNum), num ->
            when state is
                Unsafe -> (Unsafe, num)
                Init -> (Start, num)
                Start -> (isSafeDiff lastNum num, num)
                Inc | Dec -> (combineStates state (isSafeDiff lastNum num), num)
        resultState != Unsafe)
    Ok (Num.toStr safeCount)

report = when result is
    Ok res -> res
    Err str -> str

main =
    Stdout.line! report
