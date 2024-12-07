app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/07.txt" as input : Str

allowConcat = Bool.true

result =
    entries = Str.trim input |> Str.splitOn "\n" |> List.mapTry? \line ->
        split = Str.trim line |> Str.splitFirst? ": "
        val = Str.toU64? split.before
        operands = Str.splitOn split.after " " |> List.mapTry? Str.toU64
        Ok (val, operands)
    tryKeepIf? entries \entry -> canBeConstructed entry.0 entry.1
    |> List.map \entry -> entry.0
    |> List.sum
    |> Num.toStr
    |> Ok

canBeConstructed: U64, List U64 -> Result Bool [InvalidNumStr]
canBeConstructed = \val, operands ->
    when operands is
        [] -> Ok Bool.false
        [head, .. as tail] ->
            canBeConstructedWith val head tail

canBeConstructedWith: U64, U64, List U64 -> Result Bool [InvalidNumStr]
canBeConstructedWith = \val, accum, operands ->
    when operands is
        [] -> Ok (val == accum)
        [head, .. as tail] ->
            canBeAdded = canBeConstructedWith? val (accum + head) tail
            canBeMultiplied = canBeConstructedWith? val (accum * head) tail
            canBeConcatenated =
                Str.concat (Num.toStr accum) (Num.toStr head)
                |> Str.toU64
                |> Result.try? \newAccum ->
                    canBeConstructedWith val newAccum tail
            Ok (canBeAdded || canBeMultiplied || (allowConcat && canBeConcatenated))

tryKeepIf: List a, (a -> Result Bool b) -> Result (List a) b
tryKeepIf = \l, predicate ->
    List.mapTry? l \it ->
        predicate it
        |> Result.map \shouldKeep ->
            if shouldKeep then
                Ok it
            else
                Err it
    |> List.keepOks \it -> it
    |> Ok

main =
    Stdout.line! (when result is
        Ok res -> res
        Err InvalidNumStr -> "invalid num"
        Err NotFound -> "not found"
    )