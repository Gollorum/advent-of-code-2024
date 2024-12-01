app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/01.txt" as input : Str

lines = Str.trim input |> Str.splitOn "\n"
diffSum =
    entries = List.mapTry? lines \line ->
        sploot = Str.splitFirst (Str.replaceEach line "\r" "") "   "
        firstNum = Result.try sploot \s -> Str.toI32 s.before |> Result.mapErr \_e -> NaN "nan: s$(s.before)"
        secondNum = Result.try sploot \s -> Str.toI32 s.after |> Result.mapErr \_e -> NaN "nan: s$(s.after)"
        Result.try firstNum \fn -> Result.try secondNum \sn -> Ok (fn, sn)
    lefts = List.sortAsc (List.map entries \(a, _b) -> a)
    rights = List.sortAsc (List.map entries \(_a, b) -> b)
    diffs = List.map2 lefts rights \l, b -> Num.abs (l - b)
    sim1 = List.sum diffs
    scores = List.map lefts \l ->
        counts = List.countIf rights \r -> l == r
        l * Num.intCast counts
    sim2 = List.sum scores
    Ok (sim1, sim2)
res =
    when diffSum is
        Ok (sim1, sim2) -> "part1: $(Num.toStr sim1), part2: $(Num.toStr sim2)"
        Err InvalidNumStr -> "invalid num"
        Err NotFound -> "not found"
        Err (NaN str) -> str
main =
    Stdout.line! res
