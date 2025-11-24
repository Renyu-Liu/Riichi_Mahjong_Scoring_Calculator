# Riichi Mahjong Scoring Calculator
Created by Thomas (@akakrabz), Karan (@Karan-Annam), Renyu (@Renyu-Liu)
## Introduction
Riichi Mahjong has a complicated mechanism of scoring calculation. People often find it hard to calculate points manually. That's why we have introduced Riichi Mahjong Scoring Calculator. This is a Rust library that calculates the score of a winning Riichi Mahjong hand. It receives a complete input of the winning hand and game state and returns a detailed score breakdown, guiding players to redistribute their points.
## Installation
### Step 1: Install Rust (skip if already done)

Visit official Rust website [Rustup](https://rustup.rs) to download Rust. Follow instructions on the website to configure Rust locally. Run `rustc --version` in the terminal to check if Rust is installed successfully.

### Step 2: Download the program

* Option 1: Download and extract the ZIP file from [GitHub repo](https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator)

* Option 2: Clone the repository from [GitHub repo](https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator):
    
```bash
git clone https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator.git
```

Note: Make sure the the folder directly contains `Cargo.toml` file (.../Riichi_Mahjong_Scoring_Calculator/Cargo.toml). Do not move `Cargo.toml` to other folders.

### Step 3: Run the program

* Change to the program folder:
    
```bash
    cd .../Riichi_Mahjong_Scoring_Calculator
```

Note: If you renamed "Riichi_Mahjong_Scoring_Calculator", `cd` goes with your renamed folder name

* Run the program:

```bash
    cargo run
```

## User Manual

### 1: Selecting Tiles
![Selecting Tiles](lib/readme_images/Composition_Phase.jpg)

Click the tile in Tile Pool to add the tile into Hand Preview. Click the tile in Hand Preview to remove the tile.

You have to select at least 14 tiles to continue to next phase. Click "Confirm Hand" to continue.

### 2: Selecting Winning Tile

![winning tiles](lib/readme_images/winning_tile.png)

Click "Modify Hand" to return to tile selecting phase.

Click "Select" button under Winning Tile to select from your hand. You must select one winning tile to continue to next phase.

![selection_done](lib/readme_images/winning_tile_done.png)

Click "Change" to modify the winning tile.

### 3: Selecting Game States (Optional)

![open meld](lib/readme_images/open_meld.png)

Click "Add Pon/Chii/Kan" to add pon/chii/kan you made in your round. Click "Change" to modify the open meld. Click "Remove" to remove the open meld.

![select_huntsu](lib/readme_images/select_shuntsu.png)

It will display all possible pon/chii/kan from your hand. Click the meld to select pon/chii/kan you made in your round. 

![game_context](lib/readme_images/game_state.png)

You may check for seats, context-dependent yaku, dora, and honba in your round. Selecting correct game contexts yields accurate score breakdown. 

### 4: Generating Score Breakdown

Scroll down and click "Calculate Score" button to view the final score breakdown.

![score](lib/readme_images/score.png)

The score breakdown includes the total points, fu/han points, and yaku detected. It also guides players to redistribute their points.

## Technical Overview

The flowchart below shows the logic flow of Riichi Mahjong Scoring Calculator:

![flowchart](lib/readme_images/flowchart.png)

* Frontend Logic: The program directly handles all possible input conflicts according to Riichi Mahjong rules and guides users to correct their inputs. It ensures the input that sent to backend is recognizable.

* Backend Logic: The program involves multiple decision routes in the backend to detect all yaku, regular and irregular. Based on Riichi Mahjong scoring rules, there must be at least 1 yaku to calculate the score. Key crossroads include the check for hand structure and type of yaku. The final result is calculated based on the number of yaku and fu/han points.

## Challenges

* Loading speed: We face challenges in improving loading speed. We found that the loading time for the tile pool is too long, which is restricted by the image loading speed. We are working on it.

* Yaku check: Some rare yaku with complicated rules are not correctly detected. We are working on it.

## Reference

Scoring calculation is based on [standard Riichi Mahjong scoring rules](https://riichi.wiki/Japanese_mahjong_scoring_rules).

Yaku check is based on [standard Riichi Mahjong yaku lists](https://riichi.wiki/List_of_yaku).

Images of tiles are from [riichi-mahjong-tiles](https://github.com/FluffyStuff/riichi-mahjong-tiles).