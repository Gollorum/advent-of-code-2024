app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import "inputs/05.txt" as input : Str

parseDependency: Str -> Result {from: U8, to: U8} [InvalidNumStr, CannotFind Str Str]
parseDependency = \str ->
    dep = Str.splitFirst str "|"
        |> Result.mapErr? \_e -> CannotFind "|" str
    from = Str.toU8? dep.before
    to = Str.toU8? dep.after
    Ok {from: from, to:to}

insertDependency: Dict U8 (Set U8), U8, U8 -> Dict U8 (Set U8)
insertDependency = \dict, key, val ->
    Dict.update dict key \possibleSet ->
        when possibleSet is
            Err Missing -> Set.single val |> Ok
            Ok prevSet -> Set.insert prevSet val |> Ok

parseDependencies: Str -> Result (Dict U8 (Set U8)) [InvalidNumStr, CannotFind Str Str]
parseDependencies = \depsStr ->
    Str.splitOn depsStr "\n"
    |> List.mapTry? parseDependency
    |> List.walk (Dict.empty {}) \dict, {from: f, to: t} ->
        insertDependency dict f t
    |> Ok

parseUpdates: Str -> Result (List (List U8)) [InvalidNumStr]
parseUpdates = \updates ->
    Str.splitOn updates "\n"
    |> List.mapTry \update ->
        Str.splitOn update ","
        |> List.mapTry Str.toU8

isInWrongOrder: List U8, Dict U8 (Set U8) -> Bool
isInWrongOrder = \update, dependencies ->
    when update is
        [] -> Bool.false
        [head, .. as tail] ->
            isInWrongOrder tail dependencies || List.any tail \t ->
                when Dict.get dependencies t is
                    Err KeyNotFound -> Bool.false
                    Ok successors -> Set.contains successors head

orderUpdate: List U8, Dict U8 (Set U8) -> Result (List U8) [NoHeadIn (List U8)]
orderUpdate = \update, dependencies ->
    when update is
        [] -> Ok []
        _ ->
            noDepsRes = List.findFirst update \num -> !(anyIllegalDeps num update dependencies)
            noDeps = Result.mapErr? noDepsRes \_e -> NoHeadIn update
            tail = List.dropIf update \num -> num == noDeps
            List.prepend (orderUpdate? tail dependencies) noDeps
            |> Ok

anyIllegalDeps: U8, List U8, Dict U8 (Set U8) -> Bool
anyIllegalDeps = \head, tail, deps ->
    List.any tail \t ->
         t != head && when Dict.get deps t is
             Err KeyNotFound -> Bool.false
             Ok successors -> Set.contains successors head

result =
    { before: depsStr, after: updatesStr } =
        Str.trim input
        |> Str.splitFirst "\n\n"
        |> Result.mapErr? \_e -> BreakNotFound
    deps = parseDependencies? depsStr
    updates = parseUpdates? updatesStr
    updatesInCorrectOrder = List.dropIf updates \update -> isInWrongOrder update deps
    middles = List.mapTry? updatesInCorrectOrder \update ->
        List.get update (Num.floor (Num.toFrac (List.len update) / 2))
    part1 = List.map middles Num.toU64 |> List.sum
    updatesInIncorrectOrder = List.dropIf updates \update -> !(isInWrongOrder update deps)
    correctedUpdates = List.mapTry? updatesInIncorrectOrder \update -> orderUpdate update deps
    middles2 = List.mapTry? correctedUpdates \update ->
        List.get update (Num.floor (Num.toFrac (List.len update) / 2))
    part2 = List.map middles2 Num.toU64 |> List.sum
    Ok (part1, part2)

main = Stdout.line! (when result is
    Err InvalidNumStr -> "InvalidNumStr"
    Err OutOfBounds -> "Out of bounds"
    Err BreakNotFound -> "Break not found"
    Err (CannotFind val in) -> "Cannot find $(val) in $(in)"
    Err (NoHeadIn update) -> Str.concat "No head in: " (List.walk update "" \str, num -> "$(str), $(Num.toStr num)")
    Ok (part1, part2) -> "part1: $(Num.toStr part1), part2: $(Num.toStr part2)")