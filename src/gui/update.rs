use super::components::sort_tiles_by_type;
use super::messages::Message;
use super::state::{Phase, RiichiGui};
use crate::implements::calculate_agari;
use crate::implements::game::{AgariType, GameContext, PlayerContext};
use crate::implements::input::{OpenMeldInput, UserInput};
use crate::implements::tiles::{Hai, Kaze};

pub trait Update {
    fn update(&mut self, message: Message);
}

impl Update for RiichiGui {
    fn update(&mut self, message: Message) {
        match message {
            // --- Composition Phase ---
            Message::AddTile(tile) => {
                // Add tile
                if self.hand_tiles.len() < 18 {
                    let idx = crate::implements::tiles::tile_to_index(&tile);
                    if self.tile_counts[idx] > 0 {
                        self.tile_counts[idx] -= 1;
                        self.hand_tiles.push(tile);
                        self.hand_tiles.sort_by_key(sort_tiles_by_type);
                    }
                }
            }
            Message::RemoveTile(index) => {
                // Remove tile
                if index < self.hand_tiles.len() {
                    let tile = self.hand_tiles.remove(index);
                    let idx = crate::implements::tiles::tile_to_index(&tile);
                    self.tile_counts[idx] += 1;
                }
            }
            Message::ConfirmHand => {
                // go to definition phase
                if self.hand_tiles.len() >= 14 {
                    self.phase = Phase::Definition;
                }
            }
            Message::CancelSelection => {
                self.phase = Phase::Definition;
            }

            // --- Definition Phase ---
            Message::ModifyHand => {
                for meld in &self.open_melds {
                    for tile in self.get_meld_tiles(meld) {
                        self.hand_tiles.push(tile);
                    }
                }
                for kan_tile in &self.closed_kans {
                    for _ in 0..4 {
                        self.hand_tiles.push(*kan_tile);
                    }
                }
                self.hand_tiles.sort_by_key(sort_tiles_by_type);

                self.phase = Phase::Composition;
                self.winning_tile = None;
                self.open_melds.clear();
                self.closed_kans.clear();
                self.num_akadora = 0;
            }
            Message::StartSelectWinningTile => {
                self.phase = Phase::SelectingWinningTile;
            }
            Message::SelectWinningTile(tile) => {
                self.winning_tile = Some(tile);
                self.phase = Phase::Definition;
            }
            Message::SelectMeldType(m_type) => {
                self.phase = Phase::SelectingMeldTile(m_type);
            }
            Message::SelectCompleteMeld(meld) => {
                // Add a meld
                if let Phase::SelectingMeldTile(_) = self.phase {
                    if self.can_form_meld(&meld) {
                        // Remove tiles from hand
                        let tiles = self.get_meld_tiles(&meld);
                        for tile in tiles {
                            if let Some(pos) = self.hand_tiles.iter().position(|x| x == &tile) {
                                self.hand_tiles.remove(pos);
                            }
                        }

                        self.open_melds.push(meld);

                        // Reset if open hand
                        if !self.open_melds.is_empty() {
                            self.is_riichi = false;
                            self.is_daburu_riichi = false;
                            self.is_ippatsu = false;
                            self.is_tenhou = false;
                            self.is_chiihou = false;
                            self.is_renhou = false;
                        }
                    }
                }
                self.phase = Phase::Definition;
            }
            Message::StartAddClosedKan => {
                self.phase = Phase::SelectingClosedKan;
            }
            Message::SelectClosedKan(tile) => {
                // Remove 4 tiles from hand
                for _ in 0..4 {
                    if let Some(pos) = self.hand_tiles.iter().position(|x| x == &tile) {
                        self.hand_tiles.remove(pos);
                    }
                }
                self.closed_kans.push(tile);
                self.phase = Phase::Definition;
            }

            Message::RemoveOpenMeld(idx) => {
                if idx < self.open_melds.len() {
                    let meld = self.open_melds.remove(idx);
                    for tile in self.get_meld_tiles(&meld) {
                        // Return to Hand
                        self.hand_tiles.push(tile);
                        // Update counts for composition (legacy but kept for safety)
                        let t_idx = crate::implements::tiles::tile_to_index(&tile);
                        self.tile_counts[t_idx] += 1;
                    }
                    self.hand_tiles.sort_by_key(sort_tiles_by_type);
                }
            }
            Message::RemoveClosedKan(idx) => {
                if idx < self.closed_kans.len() {
                    let tile = self.closed_kans.remove(idx);
                    for _ in 0..4 {
                        self.hand_tiles.push(tile);
                    }
                    let t_idx = crate::implements::tiles::tile_to_index(&tile);
                    self.tile_counts[t_idx] += 4;
                    self.hand_tiles.sort_by_key(sort_tiles_by_type);
                }
            }

            // --- Settings Updates ---
            Message::ToggleAgariType(agari_type) => {
                self.agari_type = agari_type;
                // Reset incompatible flags
                match self.agari_type {
                    AgariType::Ron => {
                        self.is_tenhou = false;
                        self.is_chiihou = false;
                        self.is_haitei = false;
                        self.is_rinshan = false;
                    }
                    AgariType::Tsumo => {
                        self.is_renhou = false;
                        self.is_houtei = false;
                        self.is_chankan = false;
                    }
                }
            }
            Message::SetBakaze(kaze) => {
                self.bakaze = kaze;
            }
            Message::SetJikaze(kaze) => {
                self.jikaze = kaze;
                // Reset Tenhou/Chiihou
                if self.jikaze != Kaze::Ton {
                    self.is_tenhou = false;
                } else {
                    self.is_chiihou = false;
                }
            }
            Message::ToggleRiichi(is_riichi) => {
                self.is_riichi = is_riichi;
                if !self.is_riichi {
                    self.is_daburu_riichi = false;
                    self.is_ippatsu = false;
                }
            }
            Message::ToggleDoubleRiichi(is_double) => {
                self.is_daburu_riichi = is_double;
                if self.is_daburu_riichi {
                    self.is_riichi = true;
                }
            }
            Message::ToggleIppatsu(is_ippatsu) => {
                if self.is_riichi {
                    self.is_ippatsu = is_ippatsu;
                }
            }
            Message::ToggleRinshan(val) => self.is_rinshan = val,
            Message::ToggleChankan(val) => self.is_chankan = val,
            Message::ToggleHaitei(val) => self.is_haitei = val,
            Message::ToggleHoutei(val) => self.is_houtei = val,
            Message::ToggleTenhou(val) => {
                if self.open_melds.is_empty() {
                    self.is_tenhou = val;
                }
            }
            Message::ToggleChiihou(val) => {
                if self.open_melds.is_empty() {
                    self.is_chiihou = val;
                }
            }
            Message::ToggleRenhou(val) => {
                if self.open_melds.is_empty() {
                    self.is_renhou = val;
                }
            }
            Message::IncrementHonba => self.honba += 1,
            Message::DecrementHonba => {
                if self.honba > 0 {
                    self.honba -= 1
                }
            }
            Message::StartAddDora => self.phase = Phase::SelectingDora,
            Message::SelectDora(tile) => {
                self.dora_indicators.push(tile);
                self.phase = Phase::Definition;
            }
            Message::StartAddUraDora => self.phase = Phase::SelectingUraDora,
            Message::SelectUraDora(tile) => {
                self.uradora_indicators.push(tile);
                self.phase = Phase::Definition;
            }
            Message::RemoveDora(index) => {
                if index < self.dora_indicators.len() {
                    self.dora_indicators.remove(index);
                }
            }
            Message::RemoveUraDora(index) => {
                if index < self.uradora_indicators.len() {
                    self.uradora_indicators.remove(index);
                }
            }

            // --- Result Phase ---
            Message::CalculateScore => {
                if let Some(winning_tile) = self.winning_tile {
                    let mut hand_tiles = self.hand_tiles.clone();

                    // Remove winning tile if Ron
                    if self.agari_type == AgariType::Ron {
                        if let Some(pos) = hand_tiles.iter().position(|x| x == &winning_tile) {
                            hand_tiles.remove(pos);
                        }
                    }

                    let final_hand_tiles: Vec<Hai> = hand_tiles;
                    let final_open_melds: Vec<OpenMeldInput> = self.open_melds.clone();
                    let final_closed_kans: Vec<Hai> = self.closed_kans.clone();

                    let input = UserInput {
                        hand_tiles: final_hand_tiles,
                        open_melds: final_open_melds,
                        closed_kans: final_closed_kans,
                        winning_tile,
                        agari_type: self.agari_type,
                        player_context: PlayerContext {
                            jikaze: self.jikaze,
                            is_oya: self.jikaze == Kaze::Ton,
                            is_riichi: self.is_riichi,
                            is_daburu_riichi: self.is_daburu_riichi,
                            is_ippatsu: self.is_ippatsu,
                            is_menzen: self.open_melds.is_empty(),
                        },
                        game_context: GameContext {
                            bakaze: self.bakaze,
                            honba: self.honba,
                            dora_indicators: self.dora_indicators.clone(),
                            uradora_indicators: self.uradora_indicators.clone(),
                            num_akadora: self.num_akadora,
                            is_tenhou: self.is_tenhou,
                            is_chiihou: self.is_chiihou,
                            is_renhou: self.is_renhou,
                            is_haitei: self.is_haitei,
                            is_houtei: self.is_houtei,
                            is_rinshan: self.is_rinshan,
                            is_chankan: self.is_chankan,
                        },
                    };

                    match calculate_agari(&input) {
                        Ok(result) => {
                            self.score_result = Some(Ok(result));
                        }
                        Err(e) => {
                            self.score_result = Some(Err(format!("Error: {}", e)));
                        }
                    }
                    self.phase = Phase::Result;
                }
            }
            Message::StartOver => {
                self.reset();
            }
            Message::ShowRules => {
                self.show_rules = true;
            }
            Message::HideRules => {
                self.show_rules = false;
            }
            Message::IncrementAkadora => {
                if self.num_akadora < 4 {
                    self.num_akadora += 1;
                }
            }
            Message::DecrementAkadora => {
                if self.num_akadora > 0 {
                    self.num_akadora -= 1;
                }
            }
        }
    }
}
