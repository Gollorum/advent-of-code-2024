app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/03.txt" as input : Str

doBlocks = Str.splitOn input "do()" |> List.map \block ->
    when Str.splitFirst block "don't()" is
        Ok split -> split.before
        _ -> block

trimmedInput = List.walk doBlocks "" Str.concat

startWithM = Str.splitOn trimmedInput "mul("

parts =
    List.map startWithM \candidate ->
        body = Str.splitFirst? candidate ")"
        args = Str.splitFirst? body.before ","
        arg1 = Str.toU32? args.before
        arg2 = Str.toU32? args.after
        Ok (arg1 * arg2)

result =
    List.walk parts 0 \sum, maybePart ->
        when maybePart is
            Ok num -> sum + num
            _ -> sum
main =
    Stdout.line! (Num.toStr result)