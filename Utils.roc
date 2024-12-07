module [ tryKeepIf, anyTrue, joinMapWithIndex ]

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