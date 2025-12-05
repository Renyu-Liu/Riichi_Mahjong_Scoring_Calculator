# Riichi Mahjong Scoring Calculator
_Created by Renyu (@Renyu-Liu), Thomas (@akakrabz), and Karan (@Karan-Annam) as the final project for [CS 128H](https://honors.cs128.org)_
## Introduction
Riichi Mahjong has a complicated mechanism of scoring calculation. People often find it hard to calculate points manually. That's why we have introduced Riichi Mahjong Scoring Calculator. This is a Rust library that calculates the score of a winning Riichi Mahjong hand. It receives a complete input of the winning hand and game info and returns a detailed score breakdown, guiding players to redistribute their points.
## Installation

See [RUN.md](RUN.md).

## User Manual

### 1: Select Winning Hand

<img width="996" height="595" alt="image" src="https://github.com/user-attachments/assets/18e23010-3482-4c5c-bd3e-2c1288be8ebb" />

Tiles below button "Confirm Hand" is Tile Pool. Tiles above the button is Tile Preview. Click the tile in Tile Pool to add the tile into Hand Preview. Click the tile in Hand Preview to remove the tile.

You have to select at least 14 tiles to continue to next phase. Click "Confirm Hand" to continue.

<img width="953" height="159" alt="image" src="https://github.com/user-attachments/assets/dae13d06-b0ee-4f8a-bf8e-0edbe1edffc6" />

Click "Modify Hand" to return to tile selecting phase.

### 2: Select Winning Tile

<img width="955" height="125" alt="image" src="https://github.com/user-attachments/assets/06307183-bb60-4d45-81bc-860162a3580b" />

Click "Select" button under Winning Tile to select from your hand. You must select one winning tile to continue to next phase.

<img width="959" height="316" alt="image" src="https://github.com/user-attachments/assets/50338ecd-cd3a-47aa-8f8a-093e6c954f3b" />

Click the tile image to select the winning tile.

<img width="952" height="154" alt="image" src="https://github.com/user-attachments/assets/67725902-045a-44be-af48-591454d8fb2a" />

Click the image of the winning tile to modify.

### 3: Select Game Info

<img width="957" height="139" alt="image" src="https://github.com/user-attachments/assets/ea25ca8a-b4eb-462f-85dc-6101bed07155" />

Click "Add Pon/Chii/Kan" to add pon/chii/kan you made in your round. 

<img width="997" height="216" alt="image" src="https://github.com/user-attachments/assets/93efbd4b-9976-44d7-8cd0-1b253a0a7b1b" />

It will display all possible pon/chii/kan from your hand. Click the meld to select pon/chii/kan you made in your round. 

<img width="958" height="206" alt="image" src="https://github.com/user-attachments/assets/0570d14b-a261-43c2-be55-f2d96fa47dc9" />

Click the meld image to remove the selected open meld.

<img width="961" height="651" alt="image" src="https://github.com/user-attachments/assets/2a96b04b-fe48-40a3-b0b0-52b6184728da" />

You may check for seats, context-dependent yaku, and choose the number of honba and akadora in your round. Click "Add" to add a (ura)dora tile. Click the image of (ura)dora tile to remove it.

### 4: Calculate Final Scores

Scroll down and click "Calculate Score" button to view the final score breakdown.

<img width="993" height="645" alt="image" src="https://github.com/user-attachments/assets/fe861cde-521c-48f9-a07d-de9130497efd" />

The score breakdown includes the total points, fu/han points, and yaku detected. It also guides players to redistribute their points. It will show "No Yaku Found" if no yaku is detected. 

To familiarize yourself with yaku and scoring rules, you may click "Rules" button at top right corner to view them at any time.

## Technical Overview

### Data Structure

<img width="752" height="939" alt="Untitled Diagram drawio" src="https://github.com/user-attachments/assets/215981e1-450e-4c43-9f83-af0fa1036e3d" />

* Tile works as the smallest unit and makes up all related objects.

* Winning Hand and Game Info is the user input.

* Score is displayed the output.

* The program receives Winning Hand and Game Info and returns Score.

### Work Flow

<img width="964" height="1039" alt="flowchart" src="https://github.com/user-attachments/assets/b5991162-1413-4a28-9ac5-72704ff056c4" />

* Frontend Logic: The program directly handles all possible input conflicts according to Riichi Mahjong rules and guides users to correct their inputs. It ensures the input that sent to backend is recognizable.

* Backend Logic: The program involves multiple decision routes in the backend to detect all yaku, regular and irregular. Based on Riichi Mahjong scoring rules, there must be at least 1 yaku to calculate the score. Key crossroads include the check for hand structure and type of yaku. The final result is calculated based on the number of yaku and fu/han points.

## Challenges

* Some rare yaku with complicated rules are not correctly detected. We are working on it.

* We found that this app has problem running on Windows and Linux due to configuration issues. We are working on it.

## References

Scoring calculation is based on [standard Riichi Mahjong scoring rules](https://riichi.wiki/Japanese_mahjong_scoring_rules).

Yaku checker is based on [standard Riichi Mahjong yaku lists](https://riichi.wiki/List_of_yaku).

Images of tiles are from [riichi-mahjong-tiles](https://github.com/FluffyStuff/riichi-mahjong-tiles).

Image of Riichi Mahjong scoring rule is from [scoring rules sheet](https://www.reddit.com/r/Mahjong/comments/l5b221/riichi_mahjong_cheat_sheet_1_page_pdf_or_images/).

Font: [Arimo](https://fonts.google.com/specimen/Arimo)
