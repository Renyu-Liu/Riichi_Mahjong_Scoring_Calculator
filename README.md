CS-128H-Group-12-Project

Worked with help of Google Gemini

Group member names and NetIDs: @Thomas (tj29) @Karan (annam17) @Renyu Liu (renyul2) 

# Project Name: Riichi Mahjong Score Calculator

## About This Program

This is a Rust library that calculates the score of a winning Riichi Mahjong hand. It takes a complete description of the hand and game state, validates it, and returns a detailed score breakdown.

## The Logical Flow

The calculator operates in a clear sequence:

Validate Input (raw_hand_organizer.rs): First, it checks your UserInput for any logical errors (e.g., "Haitei" win on a "Ron", or "Menzen" declared with open melds). It will return an error if an impossible state is found.

Parse Hand (raw_hand_organizer.rs): It organizes the raw tile list into a standard 4-meld, 1-pair structure. If this fails, it flags the hand as "Irregular" to be checked for Kokushi Musou or Chiitoitsu.

Check Yaku (yaku_checker.rs): It finds all Yaku. It checks for Yakuman first. If no Yakuman, it finds all regular Yaku (e.g., Pinfu, Tanyao) and counts all Dora (Aka, Omote, Ura).

Calculate Score (score_calculator.rs): Finally, it converts the Yaku list into Han and calculates the Fu based on the hand's composition. It applies score limits (e.g., Mangan, Haneman) and calculates the final point payments, including Honba.

## How to Provide Input

You must provide a single UserInput struct, which contains:

hand_tiles: A Vec<Hai> of all 14+ tiles in your hand.

winning_tile: The single Hai that completed the hand.

open_melds: A list of any open calls (Chi, Pon, open Kan).

closed_kans: A list of any closed Kans (Ankan).

agari_type: How the hand was won (AgariType::Tsumo or AgariType::Ron).

player_context: Your status (dealer, seat wind, Riichi status, etc.).

game_context: The game state (round wind, Honba, Dora indicators, etc.).

## Expected Output

The program will return a Result<AgariResult, &'static str>.

On Success: AgariResult

An object containing the full score:

han: Total Han count.

fu: Total Fu count (rounded, 0 for Yakuman).

yaku_list: A list of all Yaku and Dora achieved.

limit_name: (Optional) The hand's limit, e.g., HandLimit::Mangan.

total_payment: The total points you receive.

oya_payment / ko_payment: A breakdown of who pays what.

On Failure: Error Message

A &'static str describing the error.
Example: "Invalid hand: Contains 5 or more of a single tile type."
