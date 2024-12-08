module [ tryKeepIf, anyTrue, joinMapWithIndex, appendIn, allPairs, dictAny ]

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

anyTrue: List Bool -> Bool
anyTrue = \l -> List.any l \b -> b

joinMapWithIndex: List a, (a, U64 -> List b) -> List b
joinMapWithIndex = \list, mapping ->
    List.mapWithIndex list mapping
    |> List.joinMap \it -> it

appendIn: Dict a (List b), a, b -> Dict a (List b)
appendIn = \dict, key, val ->
    Dict.update dict key \maybeList ->
        when maybeList is
            Ok list -> List.append list val |> Ok
            Err _ -> List.single val |> Ok

allPairs: List a -> List (a, a)
allPairs = \l ->
    joinMapWithIndex l \a, i ->
        List.dropFirst l (i + 1) |> List.map \b ->
            (a, b)

dictAny: Dict a b, (b -> Bool) -> Bool
dictAny = \dict, predicate ->
    Dict.walkUntil dict Bool.false \_acc, _key, value ->
        if predicate value then
            Break Bool.true
        else
            Continue Bool.false