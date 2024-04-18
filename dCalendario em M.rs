-- dCalendario em M

let
    Dt_Inicio = #date(2020, 1, 1),
    Dt_Atual = DateTime.Date(DateTime.LocalNow()),
    Duracao = Duration.From(Number.From(Dt_Atual - Dt_Inicio)),
    ListaDatas = List.Dates(Dt_Inicio, Number.RoundDown(Duration.Days(Duracao)), #duration(1, 0, 0, 0)),
    D_Calendario = Table.FromList(ListaDatas, Splitter.SplitByNothing(), {"Data"})
in
    D_Calendario