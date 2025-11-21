Todo list:

1: the whole program is way too large (1.5GB). Delete unnecessary files

2: make the program runnable on Windows

3: Improve GUI

# Project Name: Riichi Mahjong Score Calculator

## About This Program

This is a Rust library that calculates the score of a winning Riichi Mahjong hand. It takes a complete description of the hand and game state, validates it, and returns a detailed score breakdown.

## The Logical Flow

The calculator operates in a clear sequence:

Validate Input (raw_hand_organizer.rs): First, it checks your UserInput for any logical errors (e.g., "Haitei" win on a "Ron", or "Menzen" declared with open melds). It will return an error if an impossible state is found.

Parse Hand (raw_hand_organizer.rs): It organizes the raw tile list into a standard 4-meld, 1-pair structure. If this fails, it flags the hand as "Irregular" to be checked for Kokushi Musou or Chiitoitsu.

Check Yaku (yaku_checker.rs): It finds all Yaku. It checks for Yakuman first. If no Yakuman, it finds all regular Yaku (e.g., Pinfu, Tanyao) and counts all Dora (Aka, Omote, Ura).

Calculate Score (score_calculator.rs): Finally, it converts the Yaku list into Han and calculates the Fu based on the hand's composition. It applies score limits (e.g., Mangan, Haneman) and calculates the final point payments, including Honba.

## Input

todo

## Output

todo

CS 128H Group 12 Project

Worked with help from Google Gemini

Group member names and NetIDs: Thomas (tj29) Karan (annam17) Renyu (renyul2) 