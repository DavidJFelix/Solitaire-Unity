namespace SolitaireLib

open System

type Suit =
  | Clubs = 0
  | Diamonds = 1
  | Hearts = 2
  | Spades = 3


type Rank =
  | Ace = 1
  | Two = 2
  | Three = 3
  | Four = 4
  | Five = 5
  | Six = 6
  | Seven = 7
  | Eight = 8
  | Nine = 9
  | Ten = 10
  | Jack = 11
  | Queen = 12
  | King = 13

type Card = {
  rank : Rank
  suit : Suit
 }

type Color =
  | Black = 'B'
  | Red = 'R'


type GameState = {
  Tableau : List<List<Card>>
  Foundation : List<Option<Card>>
  Hand : List<Card>
  Waste : List<Card>
 }

module Solitaire =
 let StandardDeck =
  [ for suit in Enum.GetValues(typeof<Suit>) -> suit ]
  |> List.collect (fun suit ->
     [ for rank in Enum.GetValues(typeof<Rank>) ->
       { rank = rank :?> Rank; suit = suit :?> Suit }
     ]
  )

 let getSuitColor suit =
  match suit with
    | Suit.Clubs -> Ok Color.Black
    | Suit.Diamonds -> Ok Color.Red
    | Suit.Hearts -> Ok Color.Red
    | Suit.Spades -> Ok Color.Black
    | _ -> Error "invalid suit"


 let parseSuit suit =
  match suit with
  | "clubs" -> Ok Suit.Clubs
  | "diamonds" -> Ok Suit.Diamonds
  | "hearts" -> Ok Suit.Hearts
  | "spades" -> Ok Suit.Spades
  | _ -> Error "invalid suit"

 let shuffleDeck deck =
  let random = new System.Random()
  List.zip [ for _ in 1..52 -> random.NextDouble() ] deck
  |> List.sortBy (fun (order, _) -> order)
  |> List.unzip
  |> fun (_order, shuffledDeck) -> shuffledDeck
